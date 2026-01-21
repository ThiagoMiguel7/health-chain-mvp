#![cfg_attr(not(feature = "std"), no_std)]

//! Medical History pallet (HealthChain).
//!
//! Stores immutable references (hashes) to medical files and indexes them by
//! patient and doctor.
//!
//! ## Cross-pallet access
//! This pallet exposes [`MedicalHistoryAccessor`] so other pallets (e.g. a
//! reader pallet) can fetch a patient-scoped record without depending on
//! internal storage layout.

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::*;

// NOVO: Importa o módulo de tipos separado
pub mod types;
pub use types::*;

use pallet_medical_permissions::MedicalPermissionsVerifier;

/// Public interface used by external pallets (e.g. `medical-history-reader`)
/// to query a patient's record.
///
/// This trait intentionally hides the pallet's internal storage details.
///
/// # Type parameters
/// - `AccountId`: Runtime account identifier type.
/// - `Moment`: Timestamp moment type.
///
/// # Notes
/// The `file_hash` type is fixed to a bounded vector of length 64 bytes (FileHash).
pub trait MedicalHistoryAccessor<AccountId, Moment> {
    /// Fetches the current medical record belonging to `patient`.
    ///
    /// Returns `Some(record)` if the patient has an active record,
    /// otherwise `None`.
    fn get_patient_record(patient: &AccountId) -> Option<MedicalRecord<AccountId, Moment>>;
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    // NOTA: FileHash e MedicalRecord agora vêm de `use super::*;` (types.rs)

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Pallet configuration.
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        /// Weight information for extrinsics.
        type WeightInfo: WeightInfo;

        /// Permissions verifier used to authorize doctors.
        type Permissions: MedicalPermissionsVerifier<Self::AccountId>;
    }

    /// Global index: `file_hash -> record`.
    #[pallet::storage]
    #[pallet::getter(fn records)]
    pub type Records<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        FileHash,
        MedicalRecord<T::AccountId, T::Moment>,
        OptionQuery,
    >;

    /// Patient index: `(patient) -> record`.
    ///
    /// Enables patient-scoped lookup for reader pallets and patient self-access.
    #[pallet::storage]
    #[pallet::getter(fn patient_record)]
    pub type PatientRecords<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, FileHash, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new record was created and indexed.
        RecordCreated {
            /// The patient that owns the record.
            patient: T::AccountId,
            /// The doctor that created the record.
            doctor: T::AccountId,
            /// The file hash reference.
            hash: FileHash,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// A record with the same hash already exists in the global index.
        RecordAlreadyExists,
        /// Record does not exist.
        RecordNotFound,
        /// Caller is not authorized.
        NotAuthorized,
        /// Doctor does not have permission to write for this patient.
        NoPermission,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Creates a new medical record reference and indexes it.
        ///
        /// # Parameters
        /// - `origin`: Must be signed (doctor).
        /// - `patient`: Patient account that owns the record.
        /// - `file_hash`: 64-byte file hash reference.
        ///
        /// # Authorization
        /// Requires `T::Permissions::has_access(patient, doctor) == true`.
        ///
        /// # Storage
        /// - Writes: [`Records`], [`PatientRecords`]
        ///
        /// # Emits
        /// - [`Event::RecordCreated`]
        ///
        /// # Errors
        /// - [`Error::NoPermission`]: if the doctor lacks permission.
        /// - [`Error::RecordAlreadyExists`]: if `file_hash` already exists in [`Records`].
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

            // 1) Global index
            Records::<T>::insert(&file_hash, record.clone()); //TODO: verificar se esse registro é lido em alguma lógica.
                                                              //esse registro pode ser usado para dar mais detalhes de
                                                              //um histórico de paciente, como data de criação e médico responsável.

            // 2) Patient index
            PatientRecords::<T>::insert(&patient, &file_hash); //aqui não precisamos de chave composta. vamos manter apenas o file_hash mais recente com o paciente.
                                                               //isso é útlil para a consulta do histórico atual de um paciente.
                                                               // com o accountID do paciente, podemos buscar o file_hash do seu histórico atual.
                                                               // e com o file_hash, podemos buscar os detalhes do histórico na storage Records.

            Self::deposit_event(Event::RecordCreated {
                patient,
                doctor,
                hash: file_hash,
            });

            Ok(())
        }
    }

    /// Implementation of the public accessor interface used by reader pallets.
    impl<T: Config> MedicalHistoryAccessor<T::AccountId, T::Moment> for Pallet<T> {
        //faltou um teste unitário para isso?
        fn get_patient_record(
            patient: &T::AccountId,
        ) -> Option<MedicalRecord<T::AccountId, T::Moment>> {
            // Patient-scoped lookup: if it exists here, it's owned by `patient`.
            //let file_hash = PatientRecords::<T>::get(patient)?;
            //Records::<T>::get(file_hash)
            PatientRecords::<T>::get(patient).and_then(|hash| Records::<T>::get(&hash))
        }
    }
}
