#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

extern crate frame_support;
extern crate frame_system;

/// 一个简单的开始
/// 学习写一个 poe
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

/**
展开宏:
展开测试
cargo expand --tests --lib > pallet-poe.test.expand.rs
cargo expand -p pallet-poe > pallet-poe.expand.rs
*/


//把数据类型暴露出去
pub use pallet::*;




#[frame_support::pallet]
pub mod pallet {

	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*; // 包含 BoundedVec
	pub use crate::weights::WeightInfo;
	use sp_std::vec::Vec;
	//use sp_std::prelude::*;
	use core::convert::TryFrom; // BoundedVec::try_from

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		#[pallet::constant]
		type MaxClaimLength: Get<u32>;

		/// Information on runtime weights.
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);


	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	pub type Proofs<T:Config> = StorageMap<_, Blake2_128Concat, BoundedVec<u8, T::MaxClaimLength>, (T::AccountId, T::BlockNumber)>;



	/*	#[pallet::storage]
        #[pallet::getter(fn daattte)]
        pub type Daaate<T:Config> = StorageValue<Vec<T::AccountId>>;*/

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, Vec<u8>),
		ClaimRevoked(T::AccountId, Vec<u8>),
		ClaimTransfered(T::AccountId,T::AccountId, Vec<u8>), //存证转移
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		///存证已经存在
		ProofAlreadyExist,
		///存证不存在
		ProofNotExist,
		///不是存证的拥有者
		NotClaimOwner,
		///存证太长
		ClaimTooLarge,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}


	#[pallet::call]
	impl<T:Config> Pallet<T> {

		//#[pallet::weight(0)]
		#[pallet::weight(T::WeightInfo::create_claim_benchmark(claim.to_vec()))]
		pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo{
			let sender = ensure_signed(origin)?;

			/*let claim_len=claim.len();
			println!("{}",claim_len);
			if claim_len>10 {
				Err(Error::<T>::ClaimTooLarge)?;
			}*/

			//ensure!(claim.len()<=10, Error::<T>::ClaimTooLarge);

			// ensure!(
            //     T::MaxClaimLength::get() >= claim.len() as u32,
            //     Error::<T>::ClaimTooLarge
            // );

			let bounded_vec = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone()).map_err(|_|Error::<T>::ClaimTooLarge)?;

			ensure!(!Proofs::<T>::contains_key(&bounded_vec), Error::<T>::ProofAlreadyExist);

			Proofs::<T>::insert(&bounded_vec, (sender.clone(), frame_system::Pallet::<T>::block_number()));

			Self::deposit_event(Event::ClaimCreated(sender,claim));

			Ok(().into())

		}

		#[pallet::weight(0)]
		pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo{
			let sender = ensure_signed(origin)?;

			//使用ok_or,这里注销掉
			//ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ProofNotExist);

			let bounded_vec = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone()).map_err(|_|Error::<T>::ClaimTooLarge)?;

			let (owner,_) = Proofs::<T>::get(&bounded_vec).ok_or(Error::<T>::ProofNotExist)?;

			ensure!(owner==sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&bounded_vec);

			Self::deposit_event(Event::ClaimRevoked(sender,claim));

			Ok(().into())

		}

		#[pallet::weight(0)]
		pub fn transfer_claim(origin: OriginFor<T>, target:T::AccountId, claim: Vec<u8>) -> DispatchResultWithPostInfo{
			let sender = ensure_signed(origin)?;

			//使用ok_or,这里注销掉
			//ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ProofNotExist);

			let bounded_vec = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone()).map_err(|_|Error::<T>::ClaimTooLarge)?;

			let (owner,_) = Proofs::<T>::get(&bounded_vec).ok_or(Error::<T>::ProofNotExist)?;

			ensure!(owner==sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&bounded_vec); // remove 不是必须的，因为后续insert，相同的key是覆盖

			Proofs::<T>::insert(&bounded_vec, (target.clone(), frame_system::Pallet::<T>::block_number()));

			// 改变值的方法，除了insert，还可以mutate
			Proofs::<T>::mutate(&bounded_vec, |value|{
				let mut v =value.as_mut().unwrap();
				v.0 = target.clone();
				v.1 = frame_system::Pallet::<T>::block_number();
			});

			Self::deposit_event(Event::ClaimTransfered(sender,target,claim));

			Ok(().into())

		}

	}
}
