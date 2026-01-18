use crate::{mock::*, Error, Event};
use frame_support::{assert_ok, assert_noop};

#[test]
fn grant_access_works() {
	new_test_ext().execute_with(|| {
		// Avançamos para o bloco 1 para que os eventos funcionem
		System::set_block_number(1);

		let patient = 1;
		let doctor = 2;

		// 1. Tenta dar permissão
		assert_ok!(MedicalPermissions::grant_access(RuntimeOrigin::signed(patient), doctor));

		// 2. Verifica se o Evento foi emitido corretamente
		System::assert_last_event(Event::AccessGranted { patient, doctor }.into());

		// 3. Verifica se gravou no banco de dados
		assert!(MedicalPermissions::permissions(patient, doctor));
	});
}

#[test]
fn revoke_access_works() {
	new_test_ext().execute_with(|| {
		// Avançamos para o bloco 1
		System::set_block_number(1);

		let patient = 1;
		let doctor = 2;

		// Pré-requisito: Dar permissão primeiro
		assert_ok!(MedicalPermissions::grant_access(RuntimeOrigin::signed(patient), doctor));
		
		// 1. Tenta revogar
		assert_ok!(MedicalPermissions::revoke_access(RuntimeOrigin::signed(patient), doctor));

		// 2. Verifica Evento de revogação
		System::assert_last_event(Event::AccessRevoked { patient, doctor }.into());

		// 3. Verifica se o dado sumiu do banco
		assert!(!MedicalPermissions::permissions(patient, doctor));
	});
}

#[test]
fn cannot_grant_access_to_self() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let patient = 1;
		// Tentar dar permissão para si mesmo deve falhar
		assert_noop!(
			MedicalPermissions::grant_access(RuntimeOrigin::signed(patient), patient),
			Error::<Test>::SelfPermissionNotAllowed
		);
	});
}