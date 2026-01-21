//! Mock runtime for `pallet-medical-history-reader` unit tests.
//!
//! This file provides:
//! - A **mock history provider** (`MockHistoryAccessor`) that returns a single
//!   known record for `(patient = 1, file_hash = [1; 64])`.
//! - A **mock permissions provider** (`MockPermissions`) that authorizes only
//!   `(patient = 1, doctor = 10)`.
//! - A minimal FRAME test runtime wiring `System`, `Timestamp`, and
//!   `MedicalHistoryReader`.

use crate as pallet_medical_history_reader;

use frame_support::{
    derive_impl,
    traits::{ConstU32, ConstU64},
    BoundedVec,
};
use pallet_medical_history::{MedicalHistoryAccessor, MedicalRecord};
use pallet_medical_permissions::MedicalPermissionsVerifier;
use sp_runtime::BuildStorage;

type Block = frame_system::mocking::MockBlock<Test>;

// -------------------------------------------------------------------------
// Mock History Provider
// -------------------------------------------------------------------------

/// Mock implementation of the medical history accessor.
///
/// Returns a single hard-coded record only when:
/// - `patient == 1`
/// - `file_hash == [1; 64]`
pub struct MockHistoryAccessor;

impl MedicalHistoryAccessor<u64, u64> for MockHistoryAccessor {
    fn get_patient_record(patient: &u64) -> Option<MedicalRecord<u64, u64>> {
        let target_hash: BoundedVec<u8, ConstU32<64>> = vec![1; 64].try_into().unwrap();

        if *patient == 1 {
            return Some(MedicalRecord {
                created_by: 10,
                created_at: 100,
                file_hash: target_hash,
            });
        }

        None
    }
}

// -------------------------------------------------------------------------
// Mock Permissions Provider (Issue #12)
// -------------------------------------------------------------------------

/// Mock implementation of the permission verifier.
///
/// Authorization rule:
/// - Only `doctor == 10` is authorized to read data from `patient == 1`.
pub struct MockPermissions;

impl MedicalPermissionsVerifier<u64> for MockPermissions {
    fn has_access(patient: &u64, doctor: &u64) -> bool {
        *patient == 1 && *doctor == 10
    }
}

// -------------------------------------------------------------------------
// Test Runtime
// -------------------------------------------------------------------------

/// Test runtime definition.
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
    pub type MedicalHistoryReader = pallet_medical_history_reader::Pallet<Test>;
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

impl pallet_medical_history_reader::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type HistoryProvider = MockHistoryAccessor;
    type Permissions = MockPermissions;
}

/// Builds genesis storage according to the mock runtime configuration.
pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .expect("genesis storage should build")
        .into()
}
