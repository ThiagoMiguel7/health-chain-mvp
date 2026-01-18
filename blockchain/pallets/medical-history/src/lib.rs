#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::*;

use pallet_medical_permissions::MedicalPermissionsVerifier;
// CORREÇÃO: Adicionado o import necessário para a trait pública
use frame_support::BoundedVec;

// --- INTERFACE PÚBLICA PARA O LEITOR (ISSUE #11) ---
pub trait MedicalHistoryAccessor<AccountId, Moment> {
    fn get_patient_record(patient: &AccountId, file_hash: &BoundedVec<u8, frame_support::traits::ConstU32<64>>) -> Option<MedicalRecord<AccountId, Moment>>;
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    pub type FileHash = BoundedVec<u8, ConstU32<64>>;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        type WeightInfo: WeightInfo;
        type Permissions: MedicalPermissionsVerifier<Self::AccountId>;
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct MedicalRecord<AccountId, Moment> {
        pub created_by: AccountId,
        pub created_at: Moment,
        pub file_hash: FileHash,
    }

    #[pallet::storage]
    #[pallet::getter(fn records)]
    pub type Records<T: Config> =
        StorageMap<_, Blake2_128Concat, FileHash, MedicalRecord<T::AccountId, T::Moment>, OptionQuery>;

    // Indexador Médico -> Arquivos
    #[pallet::storage]
    #[pallet::getter(fn doctor_records)]
    pub type DoctorRecords<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        FileHash,
        (T::AccountId, T::Moment),
        OptionQuery,
    >;

    // NOVO (ISSUE #11): Indexador Paciente -> Arquivos
    // Permite que o paciente encontre seus próprios dados rapidamente.
    #[pallet::storage]
    #[pallet::getter(fn patient_records)]
    pub type PatientRecords<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId, // Paciente
        Blake2_128Concat,
        FileHash,     // Hash do Arquivo
        MedicalRecord<T::AccountId, T::Moment>, // Cópia do Registro
        OptionQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        RecordCreated {
            patient: T::AccountId,
            doctor: T::AccountId,
            hash: FileHash,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        RecordAlreadyExists,
        RecordNotFound,
        NotAuthorized,
        NoPermission,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(3).ref_time())]
        pub fn create_record(
            origin: OriginFor<T>,
            patient: T::AccountId,
            file_hash: FileHash,
        ) -> DispatchResult {
            let doctor = ensure_signed(origin)?;

            if !T::Permissions::has_access(&patient, &doctor) {
                return Err(Error::<T>::NoPermission.into());
            }

            ensure!(
                !Records::<T>::contains_key(&file_hash),
                Error::<T>::RecordAlreadyExists
            );

            let now = pallet_timestamp::Now::<T>::get();

            let record = MedicalRecord {
                created_by: doctor.clone(),
                created_at: now,
                file_hash: file_hash.clone(),
            };

            // 1. Grava no índice global
            Records::<T>::insert(&file_hash, record.clone());
            
            // 2. Grava no índice do médico
            DoctorRecords::<T>::insert(&doctor, &file_hash, (patient.clone(), now));

            // 3. NOVO: Grava no índice do paciente (Para ele poder ler depois)
            PatientRecords::<T>::insert(&patient, &file_hash, record);

            Self::deposit_event(Event::RecordCreated {
                patient,
                doctor,
                hash: file_hash,
            });

            Ok(())
        }
    }

    // Implementação da Interface para o Leitor usar
    impl<T: Config> MedicalHistoryAccessor<T::AccountId, T::Moment> for Pallet<T> {
        fn get_patient_record(patient: &T::AccountId, file_hash: &FileHash) -> Option<MedicalRecord<T::AccountId, T::Moment>> {
            // Busca direto no índice do paciente. 
            // Se existir aqui, significa que o registro é DELE.
            PatientRecords::<T>::get(patient, file_hash)
        }
    }
}