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

    /// Type alias for the bounded IPFS CID/file hash stored as bytes.
    ///
    /// The maximum length is 64 bytes.
    pub type FileHash = BoundedVec<u8, ConstU32<64>>;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Pallet configuration trait.
    ///
    /// This pallet depends on `frame_system` and `pallet_timestamp` to obtain the
    /// current block timestamp (`Moment`).
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        /// Weight information for extrinsics.
        type WeightInfo: WeightInfo;
    }

    // ---------------------------------------------------------------------
    // Data Structures
    // ---------------------------------------------------------------------

    /// Metadata for a medical record/exam linked to an external file (e.g., IPFS CID).
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

    /// Main store mapping a unique file hash to its record metadata.
    #[pallet::storage]
    #[pallet::getter(fn records)]
    pub type Records<T: Config> =
        StorageMap<_, Blake2_128Concat, FileHash, MedicalRecord<T::AccountId, T::Moment>, OptionQuery>;

    /// Permission store: (patient, doctor) -> has_access.
    #[pallet::storage]
    #[pallet::getter(fn permissions)]
    pub type Permissions<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, T::AccountId, bool, ValueQuery>;

    // ---------------------------------------------------------------------
    // Events
    // ---------------------------------------------------------------------

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A medical record was created successfully.
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
    // Extrinsics
    // ---------------------------------------------------------------------

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new medical record (doctor -> patient).
        ///
        /// The record is identified by a unique `file_hash` (e.g., an IPFS CID).
        /// This extrinsic stores only metadata on-chain.
        ///
        /// # Parameters
        /// - `origin`: The dispatch origin. Must be a signed account (doctor).
        /// - `patient`: The patient account that owns the record.
        /// - `file_hash`: A bounded file hash (max 64 bytes) identifying the record content.
        ///
        /// # Emits
        /// - [`Event::RecordCreated`]
        ///
        /// # Errors
        /// - [`Error::RecordAlreadyExists`]: If `file_hash` already exists in storage.
        ///
        /// # Weight
        /// This extrinsic uses a fixed base weight plus one database write.
        #[pallet::call_index(0)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
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

            // NOTE: Permission enforcement can be added here:
            // ensure!(Permissions::<T>::get(&patient, &doctor), Error::<T>::NotAuthorized);

            let record = MedicalRecord {
                created_by: doctor.clone(),
                created_at: pallet_timestamp::Now::<T>::get(),
                file_hash: file_hash.clone(),
            };

            Records::<T>::insert(&file_hash, record);

            Self::deposit_event(Event::RecordCreated {
                patient,
                doctor,
                hash: file_hash,
            });

            Ok(())
        }
    }
}