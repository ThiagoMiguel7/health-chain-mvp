use crate as pallet_medical_history;

use frame_support::{
    derive_impl,
    traits::{ConstU64},
};
use sp_runtime::BuildStorage;

type Block = frame_system::mocking::MockBlock<Test>;

/// Test runtime configuration.
///
/// This mock runtime wires `frame_system`, `pallet_timestamp`, and the
/// `pallet_medical_history` pallet to enable unit testing.
#[frame_support::runtime]
mod runtime {
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
    pub type System = frame_system::Pallet<Test>;

    #[runtime::pallet_index(1)]
    pub type Timestamp = pallet_timestamp::Pallet<Test>;

    #[runtime::pallet_index(2)]
    pub type MedicalHistory = pallet_medical_history::Pallet<Test>;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ConstU64<5>;
    type WeightInfo = ();
}

impl pallet_medical_history::Config for Test {
    type WeightInfo = ();
}

/// Build genesis storage according to the mock runtime and return a test
/// externalities environment.
///
/// The block number is set to `1` so that events can be deposited reliably.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let storage = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .expect("genesis storage should build");

    let mut ext = sp_io::TestExternalities::new(storage);
    ext.execute_with(|| {
        frame_system::Pallet::<Test>::set_block_number(1);
    });
    ext
}