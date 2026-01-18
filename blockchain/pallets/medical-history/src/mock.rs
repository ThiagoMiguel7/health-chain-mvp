use crate as pallet_medical_history;

use frame_support::{derive_impl, traits::ConstU64};
use pallet_medical_permissions::MedicalPermissionsVerifier;
use sp_runtime::BuildStorage;

type Block = frame_system::mocking::MockBlock<Test>;

// -----------------------------------------------------------------------------
// Mock Permissions
// -----------------------------------------------------------------------------

/// Mock implementation of [`MedicalPermissionsVerifier`] used by unit tests.
///
/// # Behavior
/// - Grants access **only** to the doctor with ID `10`.
/// - Any other doctor ID is denied.
///
/// # Notes
/// The `patient` parameter is ignored because this mock focuses solely on
/// exercising authorization branches in the pallet.
pub struct MockPermissions;

impl MedicalPermissionsVerifier<u64> for MockPermissions {
    fn has_access(_patient: &u64, doctor: &u64) -> bool {
        *doctor == 10
    }
}

// -----------------------------------------------------------------------------
// Test runtime
// -----------------------------------------------------------------------------

/// Mock runtime for pallet unit tests.
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
    /// Mocked permissions verifier used by `create_record`.
    type Permissions = MockPermissions;
}

// -----------------------------------------------------------------------------
// Externalities builder
// -----------------------------------------------------------------------------

/// Builds the genesis storage and returns [`sp_io::TestExternalities`]
/// configured for pallet tests.
///
/// # Side effects
/// Sets the initial block number to `1` so events work as expected.
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
