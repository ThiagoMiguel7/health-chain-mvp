use crate::{mock::*, Error, Event, MedicalRecord};
use frame_support::{assert_noop, assert_ok, traits::ConstU32, BoundedVec};

#[test]
fn create_record_works() {
    new_test_ext().execute_with(|| {
        // Set a deterministic timestamp for the test.
        Timestamp::set_timestamp(1_000);

        let doctor = 1;
        let patient = 2;

        // Simulated file hash (e.g., "123" as bytes) bounded to 64 bytes.
        let file_hash: BoundedVec<u8, ConstU32<64>> =
            vec![1u8, 2u8, 3u8].try_into().expect("valid bounded vec");

        // Create the record on-chain.
        assert_ok!(MedicalHistory::create_record(
            RuntimeOrigin::signed(doctor),
            patient,
            file_hash.clone(),
        ));

        // Ensure the correct event was emitted.
        System::assert_last_event(
            Event::RecordCreated {
                patient,
                doctor,
                hash: file_hash.clone(),
            }
            .into(),
        );

        // Issue #02: ensure the record exists in the main storage.
        let expected_record = MedicalRecord {
            created_by: doctor,
            created_at: 1_000,
            file_hash: file_hash.clone(),
        };
        assert_eq!(MedicalHistory::records(file_hash.clone()), Some(expected_record));

        // Issue #04: ensure the record appears in the doctor index.
        // Key: (doctor, file_hash) -> Value: (patient, timestamp)
        assert_eq!(
            MedicalHistory::doctor_records(doctor, file_hash),
            Some((patient, 1_000)),
        );
    });
}

#[test]
fn create_duplicate_fails() {
    new_test_ext().execute_with(|| {
        // Optional, but keeps the environment consistent/deterministic.
        Timestamp::set_timestamp(1_000);

        let doctor = 1;
        let patient = 2;

        let file_hash: BoundedVec<u8, ConstU32<64>> =
            vec![1u8, 2u8, 3u8].try_into().expect("valid bounded vec");

        // First creation should succeed.
        assert_ok!(MedicalHistory::create_record(
            RuntimeOrigin::signed(doctor),
            patient,
            file_hash.clone(),
        ));

        // Second creation with the same hash should fail.
        assert_noop!(
            MedicalHistory::create_record(RuntimeOrigin::signed(doctor), patient, file_hash),
            Error::<Test>::RecordAlreadyExists
        );
    });
}