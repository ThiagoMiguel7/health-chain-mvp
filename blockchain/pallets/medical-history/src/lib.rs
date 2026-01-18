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

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    /// Bounded file hash (e.g., an IPFS CID) stored as bytes.
    ///
    /// Maximum length: 64 bytes.
    pub type FileHash = BoundedVec<u8, ConstU32<64>>;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Pallet configuration trait.
    ///
    /// This pallet depends on `pallet_timestamp` to read the current block time.
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        /// Weight information for extrinsics.
        type WeightInfo: WeightInfo;
    }

    // ---------------------------------------------------------------------
    // Data structures
    // ---------------------------------------------------------------------

    /// Metadata for a medical record/exam linked to an external file.
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct MedicalRecord<AccountId, Moment> {
        /// Account that created the record (typically the doctor).
        pub created_by: AccountId,
        /// Timestamp when the record was created.
        pub created_at: Moment,
        /// File hash (e.g., IPFS CID) identifying the external document.
        pub file_hash: FileHash,
    }

    // ---------------------------------------------------------------------
    // Storage
    // ---------------------------------------------------------------------

    /// Main store: `file_hash -> MedicalRecord`.
    #[pallet::storage]
    #[pallet::getter(fn records)]
    pub type Records<T: Config> =
        StorageMap<_, Blake2_128Concat, FileHash, MedicalRecord<T::AccountId, T::Moment>, OptionQuery>;

    /// Doctor index: `(doctor, file_hash) -> (patient, timestamp)`.
    ///
    /// This enables queries like: "Which records were created by doctor X?"
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

    /// Access permissions: `(patient, doctor) -> has_access`.
    #[pallet::storage]
    #[pallet::getter(fn permissions)]
    pub type Permissions<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        T::AccountId,
        bool,
        ValueQuery,
    >;

    // ---------------------------------------------------------------------
    // Events
    // ---------------------------------------------------------------------

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A record was created successfully.
        ///
        /// Fields: `{ patient, doctor, hash }`.
        RecordCreated {
            /// Patient account (record owner).
            patient: T::AccountId,
            /// Doctor account (record creator).
            doctor: T::AccountId,
            /// File hash (e.g., IPFS CID).
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
    }

    // ---------------------------------------------------------------------
    // Calls (extrinsics)
    // ---------------------------------------------------------------------

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new medical record (doctor -> patient).
        ///
        /// This extrinsic stores metadata on-chain and updates a doctor-facing index.
        ///
        /// # Parameters
        /// - `origin`: Must be a signed account (doctor).
        /// - `patient`: The patient account that owns the record.
        /// - `file_hash`: Unique bounded hash (max 64 bytes) identifying the external file.
        ///
        /// # Storage
        /// - Writes: [`Records`], [`DoctorRecords`]
        ///
        /// # Emits
        /// - [`Event::RecordCreated`]
        ///
        /// # Errors
        /// - [`Error::RecordAlreadyExists`]: If `file_hash` is already present in [`Records`].
        ///
        /// # Weight
        /// Fixed base weight plus **two database writes**.
        #[pallet::call_index(0)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(2).ref_time())]
        pub fn create_record(
            origin: OriginFor<T>,
            patient: T::AccountId,
            file_hash: FileHash,
        ) -> DispatchResult {
            let doctor = ensure_signed(origin)?;

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

            Records::<T>::insert(&file_hash, record);

            DoctorRecords::<T>::insert(&doctor, &file_hash, (patient.clone(), now));

            Self::deposit_event(Event::RecordCreated {
                patient,
                doctor,
                hash: file_hash,
            });

            Ok(())
        }
    }
}