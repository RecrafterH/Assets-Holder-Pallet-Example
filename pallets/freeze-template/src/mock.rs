use polkadot_sdk::*;

use frame::{
	deps::{sp_core, sp_core::{ConstU128, ConstU64}},
	prelude::*,
	runtime::prelude::*,
	testing_prelude::*,
};

/// Balance of an account.
pub type Balance = u128;
type Block = frame::deps::frame_system::mocking::MockBlock<Test>;

use primitives::DummyFreezeReason;

use pallet_assets::Instance1;

// Configure a mock runtime to test the pallet.
#[frame_construct_runtime]
mod test_runtime {
	#[runtime::runtime]
	#[runtime::derive(
		RuntimeCall,
		RuntimeEvent,
		RuntimeError,
		RuntimeOrigin,
		RuntimeFreezeReason,
		RuntimeHoldReason,
		RuntimeSlashReason,
		RuntimeLockId,
		RuntimeTask,
		RuntimeViewFunction
	)]
	pub struct Test;

	#[runtime::pallet_index(0)]
	pub type System = frame_system;
	#[runtime::pallet_index(1)]
	pub type Balances = pallet_balances;
	#[runtime::pallet_index(2)]
	pub type Template = crate;
	#[runtime::pallet_index(3)]
	pub type Assets = pallet_assets::Pallet<Runtime, Instance1>;
	#[runtime::pallet_index(4)]
	pub type AssetsFreezer = pallet_assets_freezer::Pallet<Runtime, Instance1>;
}

#[derive_impl(frame_system::config_preludes::ParaChainDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
	type RuntimeCall = RuntimeCall;
	type Nonce = u32;
	type Block = Block;
	type Hash = sp_core::H256;
	type Hashing = BlakeTwo256;
	type AccountId = u32;
	type Lookup = IdentityLookup<Self::AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type SystemWeightInfo = ();
	type BlockWeights = ();
	type BlockLength = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type RuntimeTask = ();
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig as pallet_balances::DefaultConfig)]
impl pallet_balances::Config for Test {
	type Balance = u128;
	type AccountStore = System;
}

impl crate::Config for Test {
	type WeightInfo = ();
	type ForeignCurrency = Assets;
	type AssetsFreezer = AssetsFreezer;
}

parameter_types! {
    pub const AssetDeposit: Balance = 10;
    pub const ApprovalDeposit: Balance = 0;
    pub const StringLimit: u32 = 50;
    pub const RemoveItemsLimit: u32 = 1000;
}

impl pallet_assets::Config<pallet_assets::Instance1> for Test {
    type ApprovalDeposit = ApprovalDeposit;
    type AssetAccountDeposit = ConstU128<1>;
    type AssetDeposit = AssetDeposit;
    type AssetId = u32;
    type AssetIdParameter = codec::Compact<u32>;
    type Balance = Balance;
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = ();
    type CallbackHandle = ();
    type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<Self::AccountId>>;
    type Currency = Balances;
    type Extra = ();
    type ForceOrigin = EnsureRoot<u64>;
    type Freezer = AssetsFreezer;
	type Holder = ();
    type MetadataDepositBase = ConstU128<1>;
    type MetadataDepositPerByte = ConstU128<1>;
    type RemoveItemsLimit = RemoveItemsLimit;
    type RuntimeEvent = RuntimeEvent;
    type StringLimit = StringLimit;
    /// Rerun benchmarks if you are making changes to runtime configuration.
    type WeightInfo = ();
}

impl pallet_assets_freezer::Config<pallet_assets::Instance1> for Test {
	type RuntimeFreezeReason = DummyFreezeReason;
	type RuntimeEvent = RuntimeEvent;
} 

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> TestState {
	let mut test = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

	pallet_assets::GenesisConfig::<Test, Instance1> {
		assets: vec![(1, 0, true, 1)], 
		metadata: vec![(1, "TOKEN".into(), "TOKEN".into(), 0)],
		accounts: vec![
			(1, 0, 100),
		],
		next_asset_id: None,
	}
	.assimilate_storage(&mut test)
	.unwrap();

	test.into()
}