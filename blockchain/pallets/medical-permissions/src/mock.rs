//! Mock runtime for unit testing the Medical Permissions pallet.

use crate as pallet_medical_permissions;

use frame_support::derive_impl;
use sp_runtime::BuildStorage;

type Block = frame_system::mocking::MockBlock<Test>;

#[frame_support::runtime]
mod runtime {
    //! Test runtime that wires the pallet under test.

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

    /// System pallet.
    #[runtime::pallet_index(0)]
    pub type System = frame_system::Pallet<Test>;

    /// Pallet under test.
    #[runtime::pallet_index(1)]
    pub type MedicalPermissions = pallet_medical_permissions::Pallet<Test>;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
}

impl pallet_medical_permissions::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

/// Builds the [`sp_io::TestExternalities`] environment for unit tests.
///
/// This initializes the default system genesis storage and returns an
/// externalities instance ready for `execute_with`.
///
/// # Returns
/// A configured [`sp_io::TestExternalities`] for this mock runtime.
///
/// # Panics
/// Panics if the genesis storage cannot be built (should not happen in a
/// correctly configured mock runtime).
pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .expect("genesis storage should build")
        .into()
}