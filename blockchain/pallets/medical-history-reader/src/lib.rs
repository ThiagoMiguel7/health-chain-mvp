#![cfg_attr(not(feature = "std"), no_std)]

//! Pallet responsible for **reading medical history data**.
//!
//! This pallet provides two read-only extrinsics:
//! - Patients can read **their own** medical records.
//! - Authorized doctors can read a **patient's** medical records,
//!   provided they have explicit permission granted via the
//!   `pallet-medical-permissions` pallet.
//!
//! This pallet **does not store data**. It acts as a secure reader
//! over the data indexed and stored by `pallet-medical-history`.

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    /// Interface to access medical history records.
    use pallet_medical_history::{FileHash, MedicalHistoryAccessor};

    /// Interface to verify patient ↔ doctor permissions.
    use pallet_medical_permissions::MedicalPermissionsVerifier;

    /// The pallet type.
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configuration trait for the Medical History Reader pallet.
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>>
            + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Weight information for extrinsics.
        type WeightInfo: WeightInfo;

        /// Provider used to read medical history data.
        type HistoryProvider: MedicalHistoryAccessor<Self::AccountId, Self::Moment>;

        /// Permission system used to authorize doctor access.
        type Permissions: MedicalPermissionsVerifier<Self::AccountId>;
    }

    /// Weight functions for this pallet.
    ///
    /// These are intentionally minimal and default to zero,
    /// since this pallet performs only reads.
    pub trait WeightInfo {
        /// Weight for reading one's own data.
        fn read_own_data() -> Weight {
            Weight::zero()
        }

        /// Weight for reading a patient's data as a doctor.
        fn read_patient_data() -> Weight {
            Weight::zero()
        }
    }

    impl WeightInfo for () {}

    /// Events emitted by the Medical History Reader pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A patient successfully read their own medical record.
        OwnDataAccessed {
            patient: T::AccountId,
            file_hash: FileHash,
        },

        /// An authorized doctor successfully read a patient's medical record.
        PatientDataAccessed {
            doctor: T::AccountId,
            patient: T::AccountId,
            file_hash: FileHash,
        },
    }

    /// Errors returned by this pallet.
    #[pallet::error]
    pub enum Error<T> {
        /// The requested medical record does not exist.
        RecordNotFound,

        /// The caller does not have permission to access the record.
        AccessDenied,
    }

    /// Dispatchable calls.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Read the caller's **own** medical record.
        ///
        /// # Parameters
        /// - `origin`: Must be a signed account (the patient).
        /// - `file_hash`: Hash identifying the medical record.
        ///
        /// # Emits
        /// - [`Event::OwnDataAccessed`]
        ///
        /// # Errors
        /// - [`Error::RecordNotFound`] if the record does not exist.
        #[pallet::call_index(0)]
        #[pallet::weight(<T as Config>::WeightInfo::read_own_data())]
        pub fn read_own_data(
            origin: OriginFor<T>,
            file_hash: FileHash,
        ) -> DispatchResult {
            let patient = ensure_signed(origin)?;

            let record =
                T::HistoryProvider::get_patient_record(&patient, &file_hash)
                    .ok_or(Error::<T>::RecordNotFound)?;

            Self::deposit_event(Event::OwnDataAccessed {
                patient,
                file_hash: record.file_hash,
            });

            Ok(())
        }

        /// Read a **patient's** medical record as an authorized doctor.
        ///
        /// This call enforces explicit permission via the
        /// `pallet-medical-permissions` pallet.
        ///
        /// # Parameters
        /// - `origin`: Must be a signed account (the doctor).
        /// - `patient_id`: The patient whose data is being accessed.
        /// - `file_hash`: Hash identifying the medical record.
        ///
        /// # Emits
        /// - [`Event::PatientDataAccessed`]
        ///
        /// # Errors
        /// - [`Error::AccessDenied`] if the doctor lacks permission.
        /// - [`Error::RecordNotFound`] if the record does not exist.
        #[pallet::call_index(1)]
        #[pallet::weight(<T as Config>::WeightInfo::read_patient_data())]
        pub fn read_patient_data(
            origin: OriginFor<T>,
            patient_id: T::AccountId,
            file_hash: FileHash,
        ) -> DispatchResult {
            let doctor = ensure_signed(origin)?;

            // Permission check (Issue #12)
            if !T::Permissions::has_access(&patient_id, &doctor) {
                return Err(Error::<T>::AccessDenied.into());
            }

            let record =
                T::HistoryProvider::get_patient_record(&patient_id, &file_hash)
                    .ok_or(Error::<T>::RecordNotFound)?;

            Self::deposit_event(Event::PatientDataAccessed {
                doctor,
                patient: patient_id,
                file_hash: record.file_hash,
            });

            Ok(())
        }
    }
}