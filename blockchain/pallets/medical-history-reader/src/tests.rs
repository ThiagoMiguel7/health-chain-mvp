use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, BoundedVec};

// TESTES DO PACIENTE (Issue #11)
#[test]
fn read_own_data_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let patient_id = 1;
		let file_hash: BoundedVec<u8, _> = vec![1; 64].try_into().unwrap();

		assert_ok!(MedicalHistoryReader::read_own_data(
			RuntimeOrigin::signed(patient_id),
			file_hash.clone()
		));

		System::assert_last_event(Event::OwnDataAccessed { 
			patient: patient_id, 
			file_hash 
		}.into());
	});
}

#[test]
fn read_own_data_fails_for_wrong_patient() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let hacker_id = 99;
		let file_hash: BoundedVec<u8, _> = vec![1; 64].try_into().unwrap();

		assert_noop!(
			MedicalHistoryReader::read_own_data(
				RuntimeOrigin::signed(hacker_id),
				file_hash
			),
			Error::<Test>::RecordNotFound 
		);
	});
}

// TESTES DO MÉDICO (Issue #12)

#[test]
fn read_patient_data_works_with_permission() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		
		let doctor_id = 10; // Autorizado no MockPermissions
		let patient_id = 1;
		let file_hash: BoundedVec<u8, _> = vec![1; 64].try_into().unwrap();

		// AÇÃO: Médico lê dado do paciente
		assert_ok!(MedicalHistoryReader::read_patient_data(
			RuntimeOrigin::signed(doctor_id),
			patient_id,
			file_hash.clone()
		));

		// VERIFICAÇÃO: Evento emitido
		System::assert_last_event(Event::PatientDataAccessed { 
			doctor: doctor_id,
			patient: patient_id, 
			file_hash 
		}.into());
	});
}

#[test]
fn read_patient_data_fails_without_permission() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		
		let doctor_hacker = 99; // NÃO autorizado
		let patient_id = 1;
		let file_hash: BoundedVec<u8, _> = vec![1; 64].try_into().unwrap();

		// AÇÃO: Médico não autorizado tenta ler
		assert_noop!(
			MedicalHistoryReader::read_patient_data(
				RuntimeOrigin::signed(doctor_hacker),
				patient_id,
				file_hash
			),
			Error::<Test>::AccessDenied // Deve falhar aqui
		);
	});
}