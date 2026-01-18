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

// Importamos a interface do pallet de permissões para poder usá-la
use pallet_medical_permissions::MedicalPermissionsVerifier;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    /// Bounded file hash (e.g., an IPFS CID) stored as bytes.
    /// Maximum length: 64 bytes.
    pub type FileHash = BoundedVec<u8, ConstU32<64>>;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Pallet configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        /// Weight information for extrinsics.
        type WeightInfo: WeightInfo;

        /// Interface para verificar se um médico tem permissão de acesso aos dados do paciente.
        /// Isso conecta este pallet ao pallet-medical-permissions.
        type Permissions: MedicalPermissionsVerifier<Self::AccountId>;
    }

    // ---------------------------------------------------------------------
    // Data structures
    // ---------------------------------------------------------------------

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct MedicalRecord<AccountId, Moment> {
        pub created_by: AccountId,
        pub created_at: Moment,
        pub file_hash: FileHash,
    }

    // ---------------------------------------------------------------------
    // Storage
    // ---------------------------------------------------------------------

    #[pallet::storage]
    #[pallet::getter(fn records)]
    pub type Records<T: Config> =
        StorageMap<_, Blake2_128Concat, FileHash, MedicalRecord<T::AccountId, T::Moment>, OptionQuery>;

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

    // ---------------------------------------------------------------------
    // Events
    // ---------------------------------------------------------------------

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A record was created successfully.
        RecordCreated {
            patient: T::AccountId,
            doctor: T::AccountId,
            hash: FileHash,
        },
    }

    // ---------------------------------------------------------------------
    // Errors
    // ---------------------------------------------------------------------

    #[pallet::error]
    pub enum Error<T> {
        /// A record with the given hash already exists.
        RecordAlreadyExists,
        /// The requested record does not exist.
        RecordNotFound,
        /// The caller is not authorized to perform the requested action.
        NotAuthorized,
        /// The doctor does not have permission to add records for this patient.
        NoPermission,
    }

    // ---------------------------------------------------------------------
    // Calls (extrinsics)
    // ---------------------------------------------------------------------

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new medical record (doctor -> patient).
        #[pallet::call_index(0)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(2).ref_time())]
        pub fn create_record(
            origin: OriginFor<T>,
            patient: T::AccountId,
            file_hash: FileHash,
        ) -> DispatchResult {
            let doctor = ensure_signed(origin)?;

            // 1. Validação de Regra de Negócio (Issue #09)
            // Verifica no pallet de permissões se 'doctor' tem acesso a 'patient'.
            if !T::Permissions::has_access(&patient, &doctor) {
                return Err(Error::<T>::NoPermission.into());
            }

            // 2. Validação de Unicidade
            ensure!(
                !Records::<T>::contains_key(&file_hash),
                Error::<T>::RecordAlreadyExists
            );

            // 3. Lógica de Negócio
            let now = pallet_timestamp::Now::<T>::get();

            let record = MedicalRecord {
                created_by: doctor.clone(),
                created_at: now,
                file_hash: file_hash.clone(),
            };

            // 4. Persistência
            Records::<T>::insert(&file_hash, record);
            DoctorRecords::<T>::insert(&doctor, &file_hash, (patient.clone(), now));

            // 5. Evento
            Self::deposit_event(Event::RecordCreated {
                patient,
                doctor,
                hash: file_hash,
            });

            Ok(())
        }
    }
}