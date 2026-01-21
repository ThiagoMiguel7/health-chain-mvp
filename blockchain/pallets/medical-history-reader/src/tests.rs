//! Unit tests for `pallet-medical-history-reader`.
//!
//! Coverage:
//! - **Issue #11**: A patient can read their own medical record (`read_own_data`).
//! - **Issue #12**: A doctor can read a patient's record only if permission exists
//!   (`read_patient_data`).

use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, BoundedVec};

// -------------------------------------------------------------------------
// Patient tests (Issue #11)
// -------------------------------------------------------------------------

/// Ensures a patient can read their own data when the record exists.
#[test]
fn read_own_data_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let patient_id = 1;
        let file_hash: BoundedVec<u8, _> = vec![1; 64].try_into().unwrap();

        assert_ok!(MedicalHistoryReader::read_own_data(RuntimeOrigin::signed(
            patient_id
        )));

        System::assert_last_event(
            Event::OwnDataAccessed {
                patient: patient_id,
                file_hash,
            }
            .into(),
        );
    });
}

/// Ensures a different account cannot read another patient's record.
#[test]
fn read_own_data_fails_for_wrong_patient() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let hacker_id = 99;
        assert_noop!(
            MedicalHistoryReader::read_own_data(RuntimeOrigin::signed(hacker_id)),
            Error::<Test>::RecordNotFound
        );
    });
}

// -------------------------------------------------------------------------
// Doctor tests (Issue #12)
// -------------------------------------------------------------------------

/// Ensures a doctor can read a patient's record when permission is granted.
#[test]
fn read_patient_data_works_with_permission() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let doctor_id = 10; // Authorized in `MockPermissions`
        let patient_id = 1;
        let file_hash: BoundedVec<u8, _> = vec![1; 64].try_into().unwrap();

        assert_ok!(MedicalHistoryReader::read_patient_data(
            RuntimeOrigin::signed(doctor_id),
            patient_id
        ));

        System::assert_last_event(
            Event::PatientDataAccessed {
                doctor: doctor_id,
                patient: patient_id,
                file_hash,
            }
            .into(),
        );
    });
}

/// Ensures a doctor without permission cannot read a patient's record.
#[test]
fn read_patient_data_fails_without_permission() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let doctor_hacker = 99; // Not authorized
        let patient_id = 1;

        assert_noop!(
            MedicalHistoryReader::read_patient_data(
                RuntimeOrigin::signed(doctor_hacker),
                patient_id
            ),
            Error::<Test>::AccessDenied
        );
    });
}
