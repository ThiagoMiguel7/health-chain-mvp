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

/// Ensures permissions are **origin-scoped** and fully isolated.
///
/// # Scenario
/// - One account grants access to a second account.
/// - A different account attempts to revoke that access.
///
/// # Expected behavior
/// Revocation is applied **only** to the caller’s permission mapping:
/// - The original grant remains unchanged.
/// - The caller’s own permission mapping is updated.
///
/// # Security rationale
/// A third party must not be able to revoke permissions granted by another origin.
#[test]
fn doctor_cannot_change_patient_permission() {
    new_test_ext().execute_with(|| {

        System::set_block_number(1);

        let patient_account: u64 = 1;
        let doctor_account: u64 = 2;
        let unrelated_account: u64 = 3;

        // Initial permission grant
        assert_ok!(MedicalPermissions::grant_access(
            RuntimeOrigin::signed(patient_account),
            doctor_account,
        ));

        // Unrelated account attempts to revoke access
        assert_ok!(MedicalPermissions::revoke_access(
            RuntimeOrigin::signed(unrelated_account),
            doctor_account,
        ));

        // Original permission must remain intact
        assert!(
            MedicalPermissions::permissions(patient_account, doctor_account),
            "Permission granted by the original origin must remain unchanged"
        );

        // Revocation must apply only to the caller's mapping
        assert!(
            !MedicalPermissions::permissions(unrelated_account, doctor_account),
            "Revocation must be scoped to the caller only"
        );
    });
}

