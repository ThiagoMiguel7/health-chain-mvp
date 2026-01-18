use crate as pallet_medical_history_reader;
use frame_support::{
    derive_impl,
    traits::{ConstU64, ConstU32},
    BoundedVec,
};
use sp_runtime::BuildStorage;
use pallet_medical_history::{MedicalHistoryAccessor, MedicalRecord};

type Block = frame_system::mocking::MockBlock<Test>;

// -------------------------------------------------------------------------
// MOCK DO HISTÓRICO (O provedor de dados)
// -------------------------------------------------------------------------
pub struct MockHistoryAccessor;

impl MedicalHistoryAccessor<u64, u64> for MockHistoryAccessor {
    fn get_patient_record(patient: &u64, file_hash: &BoundedVec<u8, ConstU32<64>>) -> Option<MedicalRecord<u64, u64>> {
        // SIMULAÇÃO:
        // Vamos fingir que existe um arquivo com hash [1, 1, 1...] pertencente ao Paciente 1.
        let target_hash: BoundedVec<u8, ConstU32<64>> = vec![1; 64].try_into().unwrap();
        
        // Se o hash for igual E o paciente for o dono (1), retorna o registro.
        if *file_hash == target_hash && *patient == 1 {
            return Some(MedicalRecord {
                created_by: 10, // Médico criador
                created_at: 100,
                file_hash: target_hash,
            });
        }
        
        // Caso contrário, retorna Nada (simulando que não encontrou ou não pertence ao paciente)
        None
    }
}

// -------------------------------------------------------------------------
// CONFIGURAÇÃO DO RUNTIME DE TESTE
// -------------------------------------------------------------------------
#[frame_support::runtime]
mod runtime {
    #[runtime::runtime]
    #[runtime::derive(
        RuntimeCall,
        RuntimeEvent,
        RuntimeError,
        RuntimeOrigin,
        RuntimeFreezeReason,
        RuntimeHoldReason,
        RuntimeSlashReason,
        RuntimeLockId,
        RuntimeTask,
        RuntimeViewFunction
    )]
    pub struct Test;

    #[runtime::pallet_index(0)]
    pub type System = frame_system::Pallet<Test>;

    #[runtime::pallet_index(1)]
    pub type Timestamp = pallet_timestamp::Pallet<Test>;

    #[runtime::pallet_index(2)]
    pub type MedicalHistoryReader = pallet_medical_history_reader::Pallet<Test>;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ConstU64<5>;
    type WeightInfo = ();
}

impl pallet_medical_history_reader::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    // Conectamos o nosso Mock aqui:
    type HistoryProvider = MockHistoryAccessor;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .expect("genesis storage should build")
        .into()
}