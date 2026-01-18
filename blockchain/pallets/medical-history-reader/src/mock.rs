use crate as pallet_medical_history_reader;
use frame_support::{
    derive_impl,
    traits::{ConstU64, ConstU32},
    BoundedVec,
};
use sp_runtime::BuildStorage;
use pallet_medical_history::{MedicalHistoryAccessor, MedicalRecord};
// NOVO: Importamos a interface
use pallet_medical_permissions::MedicalPermissionsVerifier;

type Block = frame_system::mocking::MockBlock<Test>;

// -------------------------------------------------------------------------
// MOCK DO HISTÓRICO
// -------------------------------------------------------------------------
pub struct MockHistoryAccessor;

impl MedicalHistoryAccessor<u64, u64> for MockHistoryAccessor {
    fn get_patient_record(patient: &u64, file_hash: &BoundedVec<u8, ConstU32<64>>) -> Option<MedicalRecord<u64, u64>> {
        let target_hash: BoundedVec<u8, ConstU32<64>> = vec![1; 64].try_into().unwrap();
        
        if *file_hash == target_hash && *patient == 1 {
            return Some(MedicalRecord {
                created_by: 10,
                created_at: 100,
                file_hash: target_hash,
            });
        }
        None
    }
}

// -------------------------------------------------------------------------
// MOCK DE PERMISSÕES (NOVO - Issue #12)
// -------------------------------------------------------------------------
pub struct MockPermissions;

impl MedicalPermissionsVerifier<u64> for MockPermissions {
    fn has_access(patient: &u64, doctor: &u64) -> bool {
        // REGRA DO TESTE:
        // Médico 10 tem permissão para ler o Paciente 1.
        if *patient == 1 && *doctor == 10 {
            return true;
        }
        false
    }
}

// -------------------------------------------------------------------------
// RUNTIME
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
    type HistoryProvider = MockHistoryAccessor;
    // CONEXÃO (Issue #12):
    type Permissions = MockPermissions;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .expect("genesis storage should build")
        .into()
}