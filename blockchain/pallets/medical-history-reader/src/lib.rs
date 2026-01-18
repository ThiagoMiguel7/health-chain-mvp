#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    
    // Importamos a interface do Histórico
    use pallet_medical_history::{MedicalHistoryAccessor, FileHash};
    // NOVO (Issue #12): Importamos a interface de Permissões
    use pallet_medical_permissions::MedicalPermissionsVerifier;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: WeightInfo;

        // Conexão com o Banco de Dados do Histórico
        type HistoryProvider: MedicalHistoryAccessor<Self::AccountId, Self::Moment>;

        // NOVO (Issue #12): Conexão com o Sistema de Permissões
        type Permissions: MedicalPermissionsVerifier<Self::AccountId>;
    }

    pub trait WeightInfo {
        fn read_own_data() -> Weight { Weight::zero() }
        fn read_patient_data() -> Weight { Weight::zero() }
    }
    impl WeightInfo for () {}

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// O paciente leu seus próprios dados.
        OwnDataAccessed {
            patient: T::AccountId,
            file_hash: FileHash,
        },
        /// Um médico autorizado leu os dados de um paciente.
        PatientDataAccessed {
            doctor: T::AccountId,
            patient: T::AccountId,
            file_hash: FileHash,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        RecordNotFound,
        AccessDenied,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        
        /// ISSUE #11: Ler o próprio histórico.
        #[pallet::call_index(0)]
        #[pallet::weight(<T as Config>::WeightInfo::read_own_data())]
        pub fn read_own_data(
            origin: OriginFor<T>,
            file_hash: FileHash
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let record = T::HistoryProvider::get_patient_record(&who, &file_hash)
                .ok_or(Error::<T>::RecordNotFound)?;

            Self::deposit_event(Event::OwnDataAccessed {
                patient: who,
                file_hash: record.file_hash,
            });

            Ok(())
        }

        /// ISSUE #12: Médico lê histórico do paciente.
        /// Requer permissão explícita no pallet de permissões.
        #[pallet::call_index(1)]
        #[pallet::weight(<T as Config>::WeightInfo::read_patient_data())]
        pub fn read_patient_data(
            origin: OriginFor<T>,
            patient_id: T::AccountId,
            file_hash: FileHash
        ) -> DispatchResult {
            let doctor = ensure_signed(origin)?;

            // 1. Verifica Permissão (Issue #12)
            // Se NÃO tiver acesso, retorna erro AccessDenied
            if !T::Permissions::has_access(&patient_id, &doctor) {
                return Err(Error::<T>::AccessDenied.into());
            }

            // 2. Busca o registro
            let record = T::HistoryProvider::get_patient_record(&patient_id, &file_hash)
                .ok_or(Error::<T>::RecordNotFound)?;

            // 3. Emite evento de sucesso
            Self::deposit_event(Event::PatientDataAccessed {
                doctor,
                patient: patient_id,
                file_hash: record.file_hash,
            });

            Ok(())
        }
    }
}