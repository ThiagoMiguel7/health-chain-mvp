use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, BoundedVec};

// Constantes para facilitar a leitura e manutenção
const PATIENT_ID: u64 = 1;
const AUTHORIZED_DOCTOR: u64 = 10; // O único que o Mock aceita
const UNAUTHORIZED_DOCTOR: u64 = 99;

#[test]
fn create_record_works() {
	new_test_ext().execute_with(|| {
		let file_hash: BoundedVec<u8, _> = vec![1, 2, 3].try_into().unwrap();

		// Agora usamos o AUTHORIZED_DOCTOR (10)
		assert_ok!(MedicalHistory::create_record(
			RuntimeOrigin::signed(AUTHORIZED_DOCTOR),
			PATIENT_ID,
			file_hash.clone()
		));

		System::assert_last_event(Event::RecordCreated {
			patient: PATIENT_ID,
			doctor: AUTHORIZED_DOCTOR,
			hash: file_hash,
		}.into());
	});
}

#[test]
fn create_duplicate_fails() {
	new_test_ext().execute_with(|| {
		let file_hash: BoundedVec<u8, _> = vec![1, 2, 3].try_into().unwrap();

		// 1. Cria o primeiro registro com sucesso
		assert_ok!(MedicalHistory::create_record(
			RuntimeOrigin::signed(AUTHORIZED_DOCTOR),
			PATIENT_ID,
			file_hash.clone()
		));

		// 2. Tenta criar o mesmo registro de novo (deve falhar por duplicidade, não por permissão)
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

		// Médico não autorizado tenta criar
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

		assert_ok!(MedicalHistory::create_record(
			RuntimeOrigin::signed(AUTHORIZED_DOCTOR),
			PATIENT_ID,
			file_hash.clone()
		));

		System::assert_last_event(Event::RecordCreated {
			patient: PATIENT_ID,
			doctor: AUTHORIZED_DOCTOR,
			hash: file_hash,
		}.into());
	});
}