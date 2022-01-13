#![cfg_attr(not(feature = "std"), no_std)]
// ^ 上下文 attribute， When the `std` feature flag is not enabled, the macro will expand to:no_std
/*
A crate attribute is an attribute (#[...]) that applies to the enclosing context (#![...]).

This attribute must be added to the top of your crate root, thus the context is the crate itself:

#![attribute_name]
#![attribute_name(arg1, ...)]

If you are creating

a library — the crate root will be a file called lib.rs
an application — the crate root would be the primary .rs file you build. In many cases, this will be called main.rs
an integration test - the crate root is each file in tests/
an example - the crate root is each file in examples
*/

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
///
///

pub use pallet::*;
// 放外面行不行？为什么？
//use sp_runtime::traits::AccountIdConversion;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use sp_runtime::traits::AccountIdConversion;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::Printable; // 打印日志，自己实现Printable接口
	use sp_runtime::print;     // 这个是给遗留代码使用的。 参看：https://docs.substrate.io/v3/runtime/debugging/
	use sp_std::if_std;       //如果运行在std 环境下，就做一些事情，条件编译

	use frame_support::PalletId;

	/// 添加对pallet-simplestore 模块的引用，
	/// 参看：https://docs.substrate.io/v3/runtime/pallet-coupling/
	/// 参看:https://stackoverflow.com/questions/56902167/in-substrate-is-there-a-way-to-use-storage-and-functions-from-one-custom-module
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_simplestore::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type CommissionStorage: Get<PalletId>;
	}


	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	///要看到日志，运行需要带上evn参数：RUST_LOG，具体command: RUST_LOG=runtime=debug ./target/release/node-template --dev
	impl<T:Config> Printable for Error<T> {
		fn print(&self) {
			match self {
				Error::NoneValue =>{
					"NoneValue Invalid Value（不合法的值）".print();
				},
				Error::StorageOverflow =>{
					"StorageOverflow 存储溢出异常".print();
				},
				_ => {
					"未知异常".print();
				}
			}
		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			let ca : T::AccountId=T::CommissionStorage::get().into_account();


			// Update storage.
			<Something<T>>::put(something);

			// let meta = <pallet_simplestore::MetaDataStore<T>>::get();  // 这个是私有的，无法访问，所以通过meta_data这个公有方法访问

			// let meta = <pallet_simplestore::Module<T>>::meta_data();  // 老版本这里用Module,新版本用 Pallet
			let meta = <pallet_simplestore::Pallet<T>>::meta_data();
			log::info!("I get meta from a call,I print it:{:?}, ca:{:?},",meta,ca);

			let meta2 = <pallet_simplestore::Pallet<T>>::get_meta_data();
			log::info!("this is meta got from a method:{:?}",meta2);

			//wasm 日志 不行 会报错？
			//frame_support::debug::RuntimeLogger::init();
			//frame_support::debug::debug!("wasm log - called by {:?}，this is meta got from a method:{:?}", who,meta2);


			if_std!{
				// This code is only being compiled and executed when the `std` feature is enabled.
				println!("This code is only being compiled and executed when the `std` feature is enabled.");
				println!("这个代码只有在std这个feature启用的时候，才会执行！！");
				println!("the tranx caller is :{:#?}",who);
			}

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			log::info!("这里发生了错误，记录日志");

			// 要看到日志，运行需要带上evn参数：RUST_LOG，具体command: RUST_LOG=runtime=debug ./target/release/node-template --dev
			// RUST_LOG=runtime=debug cargo run  -- --dev --tmp -lruntime=debug
			print("test `cause_error`!!!!"); //打印日志  -lruntime=debug 能打印更多日志

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => {
					Error::<T>::NoneValue.print(); //写法1
					print(Error::<T>::NoneValue);  //写法2
					Err(Error::<T>::NoneValue)?
				},
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or({
						Error::<T>::StorageOverflow.print();   //打印日志
						Error::<T>::StorageOverflow
					})?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}
