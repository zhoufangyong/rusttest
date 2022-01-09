
//test runtime
//step 1 copy template\src\mock.rs、tests.rs
//step 2 edit template to poe
//step 3 add mod mock in lib.rs

//use crate as pallet_template;
use crate as pallet_kitties;  //test zhoufy
use sp_core::H256;
use frame_support::parameter_types;
//use frame_support::{parameter_types, traits::ConstU32}; //test zhoufy
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header
};
use frame_system as system;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;


pub type Balance = u128;
pub type Index = u32;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		//TemplateModule: pallet_template::{Module, Call, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Config<T>, Storage, Event<T>},
		RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage},
		KittiesModule: pallet_kitties::{Pallet, Call, Storage, Event<T>},
	}
);

impl pallet_randomness_collective_flip::Config for Test {}

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
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
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
}



parameter_types! {
    pub const ExistentialDeposit: u128 = 500;
    pub const MaxLocks: u32 = 50;
    pub const DepositBase: u32 = 1_000;
}

//impl pallet_template::Config for Test {
impl pallet_balances::Config for Test {//test zhoufy 
    type MaxLocks = MaxLocks;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Test>;
}

impl pallet_kitties::Config for Test {
    type Event = Event;//test zhoufy 
    type Randomness = RandomnessCollectiveFlip;
    type KittyIndex = Index;
    type Currency = Balances;
    type KittyDepositBase = DepositBase;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances::GenesisConfig::<Test>{
		balances: vec![(0, 100_000_000), (1, 100_000_000), (2, 100_000_000)],
	}.assimilate_storage(&mut t).unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1)); 
	ext
}