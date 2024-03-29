use crate as pallet_poe;
use frame_support::parameter_types;
use frame_support::traits::ConstU32;
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub const MAX_CLAIM_LENGTH: u32 = 5;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		PoeModule: pallet_poe::{Pallet, Call, Storage, Event<T>},
	}
);

/**
MaxClaimLengthDefinedInMock宏展开后的真相：

pub struct MaxClaimLengthDefinedInMock;
impl MaxClaimLengthDefinedInMock {
    /// Returns the value of this parameter type.
    #[allow(unused)]
    pub const fn get() -> u32 {
        5
    }
}
impl<I: From<u32>> ::frame_support::traits::Get<I> for MaxClaimLengthDefinedInMock {
    fn get() -> I {
        I::from(5)
    }
}

*/
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
	pub const MaxClaimLengthDefinedInMock: u32 = 5;  // 改为直接使用 ConstU32<64>，这个弃用，
	// MaxClaimLengthDefinedInMock 宏展开后的真相 如上注释
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::AllowAll;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();

}


impl pallet_poe::Config for Test {
	type Event = Event;
	type MaxClaimLength = ConstU32<MAX_CLAIM_LENGTH>; // 不再引用 MaxClaimLength，而是直接引用 ConstU32， BoundedVec第二个参数只能是u32。
	type WeightInfo =();

}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	// system::GenesisConfig::default().build_storage::<Test>().unwrap().into()

	let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap().into();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1)); // 生成1个块，System::assert_last_event需要，否则报错
	ext
}
