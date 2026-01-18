#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// --- CORREÇÃO: Módulos de Teste ---
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
// ----------------------------------

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    
    use pallet_medical_history::{MedicalHistoryAccessor, FileHash};

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: WeightInfo;

        // Conexão com o Banco de Dados do Histórico
        type HistoryProvider: MedicalHistoryAccessor<Self::AccountId, Self::Moment>;
    }

    pub trait WeightInfo {
        fn read_own_data() -> Weight { Weight::zero() }
    }
    impl WeightInfo for () {}

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// O paciente leu seus próprios dados com sucesso.
        OwnDataAccessed {
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

            // Chama o Histórico para buscar o registro DESTE paciente
            let record = T::HistoryProvider::get_patient_record(&who, &file_hash)
                .ok_or(Error::<T>::RecordNotFound)?;

            // Se chegou aqui, o registro existe e é dele.
            Self::deposit_event(Event::OwnDataAccessed {
                patient: who,
                file_hash: record.file_hash,
            });

            Ok(())
        }
    }
}