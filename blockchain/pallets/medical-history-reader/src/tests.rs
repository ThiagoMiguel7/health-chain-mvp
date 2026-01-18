use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn read_own_data_works() {
	new_test_ext().execute_with(|| {
		// CORREÇÃO: Avança para o bloco 1 para registrar eventos
		System::set_block_number(1);

		// DADOS:
		let patient_id = 1; // O dono do dado no Mock
		let file_hash: BoundedVec<u8, _> = vec![1; 64].try_into().unwrap(); // O hash que existe no Mock

		// AÇÃO: Paciente 1 tenta ler seu próprio arquivo
		assert_ok!(MedicalHistoryReader::read_own_data(
			RuntimeOrigin::signed(patient_id),
			file_hash.clone()
		));

		// VERIFICAÇÃO: Evento emitido
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
		
		// DADOS:
		let hacker_id = 99; // Paciente diferente
		let file_hash: BoundedVec<u8, _> = vec![1; 64].try_into().unwrap(); // Arquivo do paciente 1

		// AÇÃO: Outra pessoa tenta ler o arquivo do paciente 1
		// O Mock vai retornar None, pois o Mock só retorna se patient == 1
		assert_noop!(
			MedicalHistoryReader::read_own_data(
				RuntimeOrigin::signed(hacker_id),
				file_hash
			),
			Error::<Test>::RecordNotFound 
		);
	});
}

#[test]
fn read_own_data_fails_for_non_existent_file() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let patient_id = 1;
		let wrong_hash: BoundedVec<u8, _> = vec![2; 64].try_into().unwrap(); // Hash que não existe

		assert_noop!(
			MedicalHistoryReader::read_own_data(
				RuntimeOrigin::signed(patient_id),
				wrong_hash
			),
			Error::<Test>::RecordNotFound
		);
	});
}