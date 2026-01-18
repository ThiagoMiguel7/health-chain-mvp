use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, BoundedVec};

// -----------------------------------------------------------------------------
// Constants
// -----------------------------------------------------------------------------

/// Patient used across tests.
const PATIENT_ID: u64 = 1;

/// The only doctor ID authorized by `MockPermissions`.
const AUTHORIZED_DOCTOR: u64 = 10;

/// Any other doctor ID is considered unauthorized by `MockPermissions`.
const UNAUTHORIZED_DOCTOR: u64 = 99;

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[test]
fn create_record_works() {
    new_test_ext().execute_with(|| {
        let file_hash: BoundedVec<u8, _> = vec![1, 2, 3].try_into().unwrap();

        // Authorized doctor creates a record for the patient.
        assert_ok!(MedicalHistory::create_record(
            RuntimeOrigin::signed(AUTHORIZED_DOCTOR),
            PATIENT_ID,
            file_hash.clone()
        ));

        // Verify the expected event.
        System::assert_last_event(
            Event::RecordCreated {
                patient: PATIENT_ID,
                doctor: AUTHORIZED_DOCTOR,
                hash: file_hash,
            }
            .into(),
        );
    });
}

#[test]
fn create_duplicate_fails() {
    new_test_ext().execute_with(|| {
        let file_hash: BoundedVec<u8, _> = vec![1, 2, 3].try_into().unwrap();

        // 1) Create the first record successfully.
        assert_ok!(MedicalHistory::create_record(
            RuntimeOrigin::signed(AUTHORIZED_DOCTOR),
            PATIENT_ID,
            file_hash.clone()
        ));

        // 2) Creating the same record again must fail with duplication error
        // (not permission error, since the doctor is authorized).
        assert_noop!(
            MedicalHistory::create_record(
                RuntimeOrigin::signed(AUTHORIZED_DOCTOR),
                PATIENT_ID,
                file_hash
            ),
            Error::<Test>::RecordAlreadyExists
        );
    });
}

#[test]
fn create_record_fails_without_permission() {
    new_test_ext().execute_with(|| {
        let file_hash: BoundedVec<u8, _> = vec![1, 2, 3].try_into().unwrap();

        // Unauthorized doctor attempts to create a record.
        assert_noop!(
            MedicalHistory::create_record(
                RuntimeOrigin::signed(UNAUTHORIZED_DOCTOR),
                PATIENT_ID,
                file_hash
            ),
            Error::<Test>::NoPermission
        );
    });
}

#[test]
fn create_record_works_with_permission() {
    new_test_ext().execute_with(|| {
        let file_hash: BoundedVec<u8, _> = vec![4, 5, 6].try_into().unwrap();

        // Authorized doctor creates a record with a different hash.
        assert_ok!(MedicalHistory::create_record(
            RuntimeOrigin::signed(AUTHORIZED_DOCTOR),
            PATIENT_ID,
            file_hash.clone()
        ));

        // Verify the expected event.
        System::assert_last_event(
            Event::RecordCreated {
                patient: PATIENT_ID,
                doctor: AUTHORIZED_DOCTOR,
                hash: file_hash,
            }
            .into(),
        );
    });
}
