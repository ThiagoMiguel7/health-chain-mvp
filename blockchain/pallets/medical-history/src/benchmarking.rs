#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as MedicalHistory;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_support::BoundedVec;

// CORREÇÃO: Importamos a macro 'vec' do scale_info para funcionar no ambiente no_std
use scale_info::prelude::vec;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn create_record() {
        // 1. Setup: Define quem chama (médico) e o paciente
        let caller: T::AccountId = whitelisted_caller();
        let patient: T::AccountId = account("patient", 0, 0);
        
        // 2. Setup: Cria um hash dummy de 64 bytes
        let file_hash: FileHash = BoundedVec::try_from(vec![1u8; 64]).unwrap();

        #[extrinsic_call]
        create_record(RawOrigin::Signed(caller), patient, file_hash);
    }

    impl_benchmark_test_suite!(MedicalHistory, crate::mock::new_test_ext(), crate::mock::Test);
}