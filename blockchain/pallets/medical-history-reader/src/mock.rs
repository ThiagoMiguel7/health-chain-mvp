use crate as pallet_medical_history;

use frame_support::{
    derive_impl,
    traits::{ConstU64},
};
use sp_runtime::BuildStorage;

// Importamos a trait para criar o Mock
use pallet_medical_permissions::MedicalPermissionsVerifier;

type Block = frame_system::mocking::MockBlock<Test>;

// -------------------------------------------------------------------------
// MOCK DE PERMISSÕES
// -------------------------------------------------------------------------
// Criamos uma estrutura falsa que diz "Sim" ou "Não" para testes.
pub struct MockPermissions;

impl MedicalPermissionsVerifier<u64> for MockPermissions {
    fn has_access(_patient: &u64, doctor: &u64) -> bool {
        // REGRA DO MOCK:
        // Apenas o médico com ID 10 tem permissão.
        // Qualquer outro ID (ex: 99) será negado.
        if *doctor == 10 {
            return true;
        }
        false
    }
}

/// Test runtime configuration.
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
    pub type MedicalHistory = pallet_medical_history::Pallet<Test>;
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

impl pallet_medical_history::Config for Test {
    type WeightInfo = ();
    // Conectamos o nosso Mock aqui
    type Permissions = MockPermissions;
}

/// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let storage = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .expect("genesis storage should build");

    let mut ext = sp_io::TestExternalities::new(storage);
    ext.execute_with(|| {
        frame_system::Pallet::<Test>::set_block_number(1);
    });
    ext
}