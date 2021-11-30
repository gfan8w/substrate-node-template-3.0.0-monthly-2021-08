#![cfg_attr(not(feature = "std"), no_std)]
//Add the macro required to build both the native Rust binary (std) and the WebAssembly (no_std) binary.
//All of the pallets used in a runtime must be set to compile with the no_std features.


// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;




#[frame_support::pallet]  // 该宏 为runtime 定义了pallet
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec; // Step 3.1 will include this in `Cargo.toml`
	use sp_runtime::traits::{AtLeast32BitUnsigned, Bounded, One,Zero,Saturating, CheckedAdd, CheckedSub};
	use  frame_support::weights::Weight;
	//use frame_support::scale_info::{Type, TypeInfo};  //这个版本还不支持TypeInfo，2021-10才支持

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
	pub struct MetaData<AccountId, Balance> {
		issuance: Balance,
		minter: AccountId,
		burner: AccountId,
	}


	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		// The type used to store balances.
		type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;
	}


	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	///一个简单的数据存储，保存accountId 和 balance，在初始化的时候，会写入默认值，在每次块初始化的时候更新
	#[pallet::storage]
	#[pallet::getter(fn meta_data)]
	pub(super) type MetaDataStore<T: Config> = StorageValue<_, MetaData<T::AccountId, T::Balance>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn account)]
	pub(super) type Accounts<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, T::Balance, ValueQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_n: T::BlockNumber) -> Weight {

			log::info!("on_initialize at block:{:?}",_n);

			let mut meta = MetaDataStore::<T>::get();

			log::info!("At block:{:?},meta:{:?}",_n,meta);

			let value: T::Balance = 50u8.into();
			meta.issuance = meta.issuance.saturating_add(value);

			MetaDataStore::<T>::mutate(|mt| {
				mt.issuance=mt.issuance.saturating_add(value);
			});

			let meta2=MetaDataStore::<T>::get();


			log::info!("At block:{:?},meta issuance:{:?}",_n, meta2.issuance);

			// Add the amount to the `minter` account in storage.
			Accounts::<T>::mutate(&meta.minter, |bal| {
				*bal = bal.saturating_add(value);
			});

			T::DbWeight::get().writes(1 as Weight)

		}


	}
	// Dispatchable functions allow users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000)]
		pub fn create_claim(
			origin: OriginFor<T>,
			proof: Vec<u8>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let sender = ensure_signed(origin)?;

			log::info!("call by:{:?}",sender);

			Ok(())
		}
	}


	// Declare `admin` as type `T::AccountId`.
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub admin: T::AccountId,
	}

	// Give it a default value.
	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				admin: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			MetaDataStore::<T>::put(MetaData {
				issuance: Zero::zero(),
				minter: self.admin.clone(),
				burner: self.admin.clone(),
			});
			let v=MetaDataStore::<T>::get();
			log::info!("GenesisBuild meta:{:?}", v);
		}
	}



}














