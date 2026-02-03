#![cfg(feature = "runtime-benchmarks")]

use super::*;
use core::convert::TryInto;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use pallet_medical_history::{FileHash, Pallet as MedicalHistoryPallet};
use pallet_medical_permissions::Pallet as MedicalPermissionsPallet;

use frame_benchmarking::account;

// importar macro `vec!` e tipo `Vec` do alloc (no no_std/wasm)
use alloc::{vec, vec::Vec};

benchmarks! {
    read_own_data {
        // setup
        let caller: T::AccountId = whitelisted_caller();

        // criar um FileHash (BoundedVec<u8, ConstU32<64>>)
        let file_hash: FileHash = {
            let bytes: Vec<u8> = vec![1u8; 64];
            bytes.try_into().expect("FileHash must be 64 bytes")
        };

        // preparar estado: inserir o registro no pallet medical-history
        MedicalHistoryPallet::<T>::bench_insert_record(&caller, &file_hash);
    }: _(RawOrigin::Signed(caller.clone()), file_hash.clone())


    read_patient_data {
        // Setup: paciente + médico habilitado + registro existente
        let doctor: T::AccountId = whitelisted_caller(); // test caller = doctor
        let patient: T::AccountId = account("p", 0, 0);
        let file_hash: FileHash = {
            let bytes: Vec<u8> = vec![2u8; 64];
            bytes.try_into().expect("FileHash must be 64 bytes")
        };

        // Insere registro no pallet medical-history (para o patient)
        MedicalHistoryPallet::<T>::bench_insert_record(&patient, &file_hash);

        // Concede permissao doctor <- patient (via helper do pallet medical-permissions)
        MedicalPermissionsPallet::<T>::bench_grant_permission(&patient, &doctor);

    }: _(RawOrigin::Signed(doctor.clone()), patient.clone(), file_hash.clone())

    verify {
        // opcional
    }
}
