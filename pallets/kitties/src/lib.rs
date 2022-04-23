#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

extern crate frame_support;
extern crate frame_system;


use codec::{Decode, Encode};
use sp_runtime::{RuntimeDebug};
use sp_std::vec::Vec;

#[cfg(feature = "std")]
use serde::{Deserializer, Deserialize, Serialize};
/// hello kitty
/// 运行效果： https://www.awesomescreenshot.com/video/4968603?key=f0b4d770c81f67d52a1d00174d55e9dc
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

//把数据类型暴露出去
pub use pallet::*;


#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum KittyStatus {
	///正常
	Active,
	///饥饿
	Hunger,
	///挂单
	Bid,
	///死亡
	Death,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ColorKitty<AccountId,BlockNumber, Balance> {
	pub owner: AccountId,
	pub birth: BlockNumber,
	pub eat_count: BlockNumber,
	pub status: KittyStatus,
	pub children: Balance,
	pub species:Balance,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct WorldKitty<AccountId,BlockNumber, Balance> {
	pub owner: AccountId,
	pub start: BlockNumber,
	pub pre_eat_at: BlockNumber,
	pub eat_count: u32,
	pub status: KittyStatus,
	pub asset_id: u64,
	pub class_id: u32,
	pub grow_value: Balance,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AsiaKitty<AccountId,BlockNumber, Balance> {
	pub owner: AccountId,
	pub start: BlockNumber,
	pub pre_eat_at: BlockNumber,
	pub eat_count: u32,
	pub eggs: Balance,
	pub status: KittyStatus,
	pub asset_id: u64,
	pub class_id: u32,
	pub incubation_remain: Balance,
}


#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Farm<AccountId, BlockNumber, Balance> {
	Red(ColorKitty<AccountId, BlockNumber, Balance>),
	World(WorldKitty<AccountId, BlockNumber, Balance>),
	Asia(AsiaKitty<AccountId, BlockNumber, Balance>),
}



#[derive(Encode, Decode, Default, RuntimeDebug)]
pub struct HackerNewsInfo {
	// Specify our own deserializing function to convert JSON string to vector of bytes
	//#[serde(deserialize_with = "de_string_to_bytes")]
	by: Vec<u8>,
	//#[serde(deserialize_with = "de_string_to_bytes")]
	title: Vec<u8>,
	//#[serde(deserialize_with = "de_string_to_bytes")]
	url: Vec<u8>,
	descendants: u32,
}

/*pub fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
	where
		D: Deserializer<'de>,
{
	let s: &str = Deserialize::deserialize(de)?;
	Ok(s.as_bytes().to_vec())
}*/


#[frame_support::pallet]
pub mod pallet {

	use super::{Farm,ColorKitty};

	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*,
						traits::Randomness,
						traits::Currency,
						traits::ReservableCurrency,
						traits::ExistenceRequirement
	};
	use frame_system::pallet_prelude::*;
	use codec::{Encode,Decode};
	use sp_core::{U256,crypto::{AccountId32}};
	use sp_io::hashing::blake2_128;
	use sp_runtime::traits::{AtLeast32BitUnsigned, Bounded, One};
	use sp_std::vec::Vec;
	use sp_std::convert::{TryFrom, TryInto};
	use sp_runtime::SaturatedConversion;
	//use sp_std::prelude::*;

	/*
	浏览器的setting/json里要加上
	{
	  "KittyIndex": "u32",
	  "Kitty": "[u8;16]"
	}
	*/


	pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

	pub type FarmOf<T> = Farm<AccountIdOf<T>, BlockNumberOf<T>, BalanceOf<T>>;

	pub type ColorKittyOf<T> = ColorKitty<AccountIdOf<T>, BlockNumberOf<T>, BalanceOf<T>>;


	#[derive(Encode,Decode)]
	pub struct Kitty(pub [u8;16]);

	type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	// 该定义被移动到 runtime里
	// type KittyIndex =u32;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// 这个Event类型可以转换成System模块下的的Event，也可以由当前的template模块定义的Event转换而来。
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;

		// 定义 KittyIndex 类型，要求实现指定的 trait
		type KittyIndex: Parameter + Member + AtLeast32BitUnsigned  + Default + Copy;

		// 创建Kitty需要质押数量
		type KittyReserve:Get<BalanceOf<Self>>;

		// Currency 类型，用于质押等于资产相关的操作
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

		//一个目标地址，通过在runtime中指定，从外部注入一个账号
		type CharityDest: Get<<Self as frame_system::Config>::AccountId>;

	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// when a kitty was created, the message will be sent
		/// 当一只kitty🐈被创建的时候，会发出这个消息
		KittyCreate(T::AccountId, T::KittyIndex),

		/// when a kitty was give to her/him, the message will be sent
		/// 当一只kitty被转移给别人的时候，会发出这个消息
		KittyTransfer(T::AccountId, T::AccountId, T::KittyIndex),
		/// WHEN a KITTY is for sale, the message will be sent.
		/// 当有一只 kitty被卖出时，您会看到这条消息
		KittyForSale(T::AccountId, T::KittyIndex, Option<BalanceOf<T>>),
		/// WHEN ALL of the KITTY are sold out, you can see the message.
		/// 当所有的🐈都被售完了，你会看到这个消息，🍋
		KittySaleOut(T::AccountId, T::KittyIndex, Option<BalanceOf<T>>),
	}

	#[pallet::storage]
	#[pallet::getter(fn kitties_count)]
	pub type KittiesCount<T: Config> = StorageValue<_, T::KittyIndex, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<Kitty>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties_park)]
	pub type KittiesPark<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, Kitty, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn owner)]
	pub type Owner<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_prices)]
	pub type KittyPrices<T: Config> =
	StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<BalanceOf<T>>, ValueQuery>;

	///仓库
	#[pallet::storage]
	#[pallet::getter(fn query_kitty_farm)]
	pub type ColorKittyFarm<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::KittyIndex,
		FarmOf<T>,
		OptionQuery,
	>;

	///仓库
	#[pallet::storage]
	#[pallet::getter(fn query_kitty_farm_map)]
	pub type ColorKittyFarmMap<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::KittyIndex,
		ColorKittyOf<T>,
		OptionQuery,
	>;


	#[pallet::storage]
	#[pallet::getter(fn hack_news)]
	pub type HackerNews<T> = StorageValue<_, Vec<super::HackerNewsInfo>, ValueQuery>;





	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		///数量太多，溢出
		KittiesCountOverflow,
		NotOwner,
		SameParentIndex,
		InvalidKittyIndex,
		MoneyNotEnough,
		AlreadyOwned,
		NotForSale,
	}


	#[pallet::call]
	impl<T:Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			//比较2个账号是否相等
			// 参考：https://stackoverflow.com/questions/65507360/how-do-i-compare-the-trait-type-with-the-string-type-in-substrate-module
			let account_bytes: Vec<u8> = who.encode();
			let match_bytes: Vec<u8> = hex_literal::hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into();

			let account: AccountId32 = hex_literal::hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into();
			let ss58_account = "";//account.to_ss58check();

			log::info!("account:{:?}, ss58:{:?}",account,ss58_account);

			if account_bytes == match_bytes {
				log::info!("account match")
			}

			// let my_value = <T::BlockNumber as As<u64>>::sa(0);

			let charity_dest = T::CharityDest::get();
			log::info!("CharityDest：{:?}",charity_dest);

			// 质押资产
			T::Currency::reserve(&who, T::KittyReserve::get())
				.map_err(|_| Error::<T>::MoneyNotEnough)?;

			let kitty_id = Self::next_kitty_id()?;

			/*let kitty_id = match Self::kitties_count() {
				Some(id) => {
					ensure!(id!=KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
					id
				},
				None => {
					0
				}
			};*/

			let dna = Self::random_value(&who);

			Kitties::<T>::insert(kitty_id,Some(Kitty(dna)));

			Owner::<T>::insert(kitty_id, Some(who.clone()));

			KittiesCount::<T>::put(kitty_id+One::one());

			Self::deposit_event(Event::KittyCreate(who.clone(),kitty_id));

			let block = frame_system::Pallet::<T>::block_number();

			KittiesPark::<T>::insert(kitty_id,Kitty(dna));

			ColorKittyFarmMap::<T>::insert(kitty_id,ColorKittyOf::<T> {
				owner: <T as frame_system::Config>::AccountId::default(),
				birth: block,
				eat_count: block,
				status: super::KittyStatus::Active,
				children: 3u32.into(),
				species: 4u32.into()
			});


			ColorKittyFarm::<T>::insert(
				kitty_id,
				FarmOf::<T>::Red(super::ColorKitty {
					owner: who.clone(),
					birth: block,
					eat_count: block,
					status: super::KittyStatus::Active,
					children: 3u32.into(),
					species:4u32.into(),
				}),
			);
			/*ColorKittyFarm::<T>::insert(
				kitty_id+ T::KittyIndex::from(10u32),
				FarmOf::<T>::World(super::WorldKitty {
					owner: who.clone(),
					start: block,
					pre_eat_at: block,
					eat_count: 2u32,
					status: super::KittyStatus::Bid,
					asset_id: 300u64,
					class_id: 30u32,
					grow_value: 3u32.into(),
				}),
			);*/
			ColorKittyFarm::<T>::insert(
				kitty_id+T::KittyIndex::from(20u32),
				FarmOf::<T>::Asia(super::AsiaKitty {
					owner: who.clone(),
					start: block,
					pre_eat_at: block,
					eat_count: 5u32,
					eggs: 3u32.into(),
					status: super::KittyStatus::Active,
					asset_id: 0,
					class_id: 0,
					incubation_remain: 12u32.into()
				}),
			);


			let content= "{\"cfd\":\"上海\",\"czr\":\"系统管理员\",\"zhi\":\"京666\"}";
			//let con = sp_std::str::from_str(content).unwrap();
			let vc=content.as_bytes().to_vec();
			let news= super::HackerNewsInfo {
				by: "author".as_bytes().to_vec(),
				title: content.as_bytes().to_vec(),
				url: "http://www.baidu.com".as_bytes().to_vec(),
				descendants: 0
			};

			HackerNews::<T>::mutate(|hn|{
				hn.push(news)
			});


			let balance = Self::trans_into_balance(43);
			let money= Self::trans_from_balance(<BalanceOf<T>>::max_value());
			let u64_max =u64::MAX;
			log::info!("money is :{:?}, max of u64 is :{:?}", money, u64_max);

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn transfer(origin: OriginFor<T>, her: T::AccountId, kitty_id: T::KittyIndex) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Some(who.clone()) == Owner::<T>::get(kitty_id), Error::<T>::NotOwner);

			ensure!(
                Some(who.clone()) != Some(her.clone()),
                Error::<T>::AlreadyOwned
            );

			// 新拥有者质押资产
			T::Currency::reserve(&her, T::KittyReserve::get())
				.map_err(|_| Error::<T>::MoneyNotEnough)?;
			// 解除原质押资产
			T::Currency::unreserve(&who, T::KittyReserve::get());

			Owner::<T>::insert(kitty_id, Some(her.clone()));

			Self::deposit_event(Event::KittyTransfer(who,her,kitty_id));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn breed(origin: OriginFor<T>, kitty_id_mom: T::KittyIndex, kitty_id_dad: T::KittyIndex) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(kitty_id_mom != kitty_id_dad, Error::<T>::SameParentIndex);

			let kitty_d=Self::kitties(kitty_id_dad).ok_or(Error::<T>::InvalidKittyIndex)?;
			let kitty_m=Self::kitties(kitty_id_mom).ok_or(Error::<T>::InvalidKittyIndex)?;

			/*let kitty_child = match Self::kitties_count() {
				Some(id) => {
					ensure!(id!=KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
					id
				},
				None => {
					0
				}
			};*/

			let kitty_child = Self::next_kitty_id()?;

			let dna_mom= kitty_d.0;
			let dna_dad=kitty_m.0;

			let selector = Self::random_value(&who);
			let mut new_dna =[0u8; 16];

			for i in 0..dna_dad.len(){
				new_dna[i]= (selector[i] & dna_dad[i]) | (!selector[i] & dna_mom[i]);
			}

			// 质押资产
			T::Currency::reserve(&who, T::KittyReserve::get())
				.map_err(|_| Error::<T>::MoneyNotEnough)?;

			Kitties::<T>::insert(kitty_child,Some(Kitty(new_dna)));

			Owner::<T>::insert(kitty_child, Some(who.clone()));

			KittiesCount::<T>::put(kitty_child+One::one());


			Self::deposit_event(Event::KittyCreate(who, kitty_child));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn buy(origin: OriginFor<T>, kitty_id: T::KittyIndex) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let kitty_owner = Owner::<T>::get(kitty_id).ok_or(Error::<T>::NotOwner)?;
			let kitty_price = KittyPrices::<T>::get(kitty_id).ok_or(Error::<T>::NotForSale)?;
			ensure!(
                Some(who.clone()) != Some(kitty_owner.clone()),
                Error::<T>::AlreadyOwned
            );

			//转账（购买）
			T::Currency::transfer(
				&who,
				&kitty_owner,
				kitty_price,
				ExistenceRequirement::KeepAlive,
			)?;

			// 新拥有者质押资产
			T::Currency::reserve(&who, T::KittyReserve::get())
				.map_err(|_| Error::<T>::MoneyNotEnough)?;
			// 解除原质押资产
			T::Currency::unreserve(&kitty_owner, T::KittyReserve::get());

			//更改拥有人
			Owner::<T>::insert(kitty_id, Some(who.clone()));

			//移除挂售
			KittyPrices::<T>::remove(kitty_id);

			Self::deposit_event(Event::KittySaleOut(who, kitty_id, Some(kitty_price)));
			Ok(())


			/*
			这个才是正确的做法
			// 判断是否是目标kitty的拥有者
			ensure!(Some(who.clone()) != Owner::<T>::get(kitty_id), Error::<T>::AlreadyOwned);

			// 检查kitty是否存在,并获取该kitty的owner
			let owner = Owner::<T>::get(kitty_id).ok_or(Error::<T>::InvalidKittyIndex)?;

			let kitty_price = KittyPrices::<T>::get(kitty_id).ok_or(Error::<T>::NotForSale)?;

			// 转质押 + 扣款
			// 对于购买者，先质押购买的和创建抵押的
			T::Currency::reserve(&who, T::KittyReserve::get() + kitty_price).map_err(|_| Error::<T>::MoneyNotEnough)?;
			// 释放卖出者质押的代币
			T::Currency::unreserve(&owner, T::KittyReserve::get());

			// 释放购买者需要支付用来质押的代币
			T::Currency::unreserve(&who, kitty_price);
			// 转账
			T::Currency::transfer(&who, &owner, kitty_price, ExistenceRequirement::KeepAlive)?;

			// 移除价格挂单
			KittyPrices::<T>::remove(kitty_id);

			// 转移Kitty
			Owner::<T>::insert(kitty_id, Some(who.clone()));

			Self::deposit_event(Event::KittySaleOut(who, kitty_id, Some(kitty_price)));

			Ok(())*/

			//refer: https://github.com/GreatMartial/substrate-advanced-course/blob/main/pallets/kitties/src/lib.rs

		}


		#[pallet::weight(0)]
		pub fn sale(
			origin: OriginFor<T>,
			kitty_id: T::KittyIndex,
			sale_price: Option<BalanceOf<T>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(
                Some(who.clone()) == Owner::<T>::get(kitty_id),
                Error::<T>::NotOwner
            );

			KittyPrices::<T>::insert(kitty_id, sale_price);

			Self::deposit_event(Event::KittyForSale(who, kitty_id, sale_price));
			Ok(())
		}


	}

	impl<T:Config> Pallet<T> {
		fn random_value(sender: &T::AccountId) ->[u8; 16] {
			let payload =(
				T::Randomness::random_seed(),
				&sender,
				<frame_system::Pallet<T>>::extrinsic_index(),
			);
			payload.using_encoded(blake2_128)
		}

		// 获取当前Kitty_id (从0开始)
		fn next_kitty_id() -> sp_std::result::Result<T::KittyIndex, DispatchError> {
			let kitty_id = Self::kitties_count();
			//ensure!(kitty_id!=T::KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
			if kitty_id == T::KittyIndex::max_value() {
				return Err(Error::<T>::KittiesCountOverflow.into());
			}
			Ok(kitty_id)
		}

		/// u32,u64等rust基本类型与SubStrate系统内的一些基本类型的数据转换。
		/// SaturatedConversion 就是用来干这个的。
		/// sp_runtime::SaturatedConversion 是对  sp_arithmetic::traits::SaturatedConversion 的重新导出
		fn trans_into_balance(money: u64) -> BalanceOf<T> {
			// u8,u32,u64 与 T::Balance的转换
			let result_balance: BalanceOf<T> = money.saturated_into::<BalanceOf<T>>();
			log::info!("result_balance:{:?}", result_balance);
			result_balance
		}

		/// u32,u64等rust基本类型与SubStrate系统内的一些基本类型的数据转换。
		/// SaturatedConversion 就是用来干这个的。
		/// sp_runtime::SaturatedConversion 是对  sp_arithmetic::traits::SaturatedConversion 的重新导出
		fn trans_from_balance(balance: BalanceOf<T>) -> u64 {
			// u8,u32,u64 与 T::Balance的转换
			let result_u64: u64 = balance.saturated_into::<u64>();
			log::info!("result_u64:{:?}", result_u64);
			result_u64
		}

	}
}
