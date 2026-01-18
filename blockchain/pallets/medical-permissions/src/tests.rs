//! Unit tests for the Medical Permissions pallet.

use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

/// Ensures a patient can grant access to a doctor, emits the expected event,
/// and persists the permission in storage.
#[test]
fn grant_access_works() {
    new_test_ext().execute_with(|| {
        // Move to block 1 so events are recorded.
        System::set_block_number(1);

        let patient = 1;
        let doctor = 2;

        // 1) Grant permission.
        assert_ok!(MedicalPermissions::grant_access(
            RuntimeOrigin::signed(patient),
            doctor
        ));

        // 2) Verify the emitted event.
        System::assert_last_event(Event::AccessGranted { patient, doctor }.into());

        // 3) Verify the storage write.
        assert!(MedicalPermissions::permissions(patient, doctor));
    });
}

/// Ensures a patient can revoke access previously granted to a doctor,
/// emits the expected event, and removes the permission from storage.
#[test]
fn revoke_access_works() {
    new_test_ext().execute_with(|| {
        // Move to block 1 so events are recorded.
        System::set_block_number(1);

        let patient = 1;
        let doctor = 2;

        // Precondition: grant permission first.
        assert_ok!(MedicalPermissions::grant_access(
            RuntimeOrigin::signed(patient),
            doctor
        ));

        // 1) Revoke permission.
        assert_ok!(MedicalPermissions::revoke_access(
            RuntimeOrigin::signed(patient),
            doctor
        ));

        // 2) Verify the emitted event.
        System::assert_last_event(Event::AccessRevoked { patient, doctor }.into());

        // 3) Verify the storage removal.
        assert!(!MedicalPermissions::permissions(patient, doctor));
    });
}

/// Ensures a patient cannot grant permission to themselves.
#[test]
fn cannot_grant_access_to_self() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let patient = 1;

        // Granting access to self should fail.
        assert_noop!(
            MedicalPermissions::grant_access(RuntimeOrigin::signed(patient), patient),
            Error::<Test>::SelfPermissionNotAllowed
        );
    });
}
