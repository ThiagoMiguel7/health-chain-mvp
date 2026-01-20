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

    /// Main pallet struct.
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Pallet configuration trait.
    ///
    /// Defines the types and dependencies required by the
    /// Medical Permissions pallet.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching runtime event type.
        ///
        /// ⚠️ Note: This associated type is deprecated in newer
        /// Polkadot SDK versions, but kept here intentionally
        /// to preserve the current logic and structure.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Weight information for extrinsics.
        type WeightInfo: WeightInfo;
    }

    // ---------------------------------------------------------------------
    // Storage
    // ---------------------------------------------------------------------

    /// Mapping of medical access permissions.
    ///
    /// `(patient, doctor) -> has_access`
    ///
    /// If the key does not exist, the value defaults to `false`
    /// due to the use of `ValueQuery`.
    #[pallet::storage]
    #[pallet::getter(fn permissions)]
    pub type Permissions<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId, // Patient
        Blake2_128Concat,
        T::AccountId, // Doctor
        bool,         // Access granted?
        ValueQuery,
    >;

    // ---------------------------------------------------------------------
    // Events
    // ---------------------------------------------------------------------

    /// Events emitted by the Medical Permissions pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Access was granted from a patient to a doctor.
        AccessGranted {
            patient: T::AccountId,
            doctor: T::AccountId,
        },

        /// Access was revoked from a patient to a doctor.
        AccessRevoked {
            patient: T::AccountId,
            doctor: T::AccountId,
        },
    }

    // ---------------------------------------------------------------------
    // Errors
    // ---------------------------------------------------------------------

    /// Errors returned by the Medical Permissions pallet.
    #[pallet::error]
    pub enum Error<T> {
        /// A patient attempted to grant permission to themselves.
        SelfPermissionNotAllowed,
    }

    // ---------------------------------------------------------------------
    // Calls (extrinsics)
    // ---------------------------------------------------------------------

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Grants read access to a doctor.
        ///
        /// # Parameters
        /// - `origin`: Must be a signed account representing the patient.
        /// - `doctor`: The doctor account that will receive access.
        ///
        /// # Storage
        /// - Writes to [`Permissions`]
        ///
        /// # Emits
        /// - [`Event::AccessGranted`]
        ///
        /// # Errors
        /// - [`Error::SelfPermissionNotAllowed`] if `patient == doctor`
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn grant_access(origin: OriginFor<T>, doctor: T::AccountId) -> DispatchResult {
            let patient = ensure_signed(origin)?;

            ensure!(patient != doctor, Error::<T>::SelfPermissionNotAllowed);

            Permissions::<T>::insert(&patient, &doctor, true);

            Self::deposit_event(Event::AccessGranted { patient, doctor });

            Ok(())
        }

        /// Revokes read access from a doctor.
        ///
        /// # Parameters
        /// - `origin`: Must be a signed account representing the patient.
        /// - `doctor`: The doctor account that will lose access.
        ///
        /// # Storage
        /// - Writes to [`Permissions`]
        ///
        /// # Emits
        /// - [`Event::AccessRevoked`]
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn revoke_access(origin: OriginFor<T>, doctor: T::AccountId) -> DispatchResult {
            let patient = ensure_signed(origin)?;

            Permissions::<T>::remove(&patient, &doctor);

            Self::deposit_event(Event::AccessRevoked { patient, doctor });

            Ok(())
        }
    }
}

// -------------------------------------------------------------------------
// Public interface (cross-pallet)
// -------------------------------------------------------------------------

/// Public verifier interface for checking whether a doctor
/// has access to a patient's medical data.
///
/// Other pallets can depend on this trait to enforce
/// authorization rules without directly accessing storage.
pub trait MedicalPermissionsVerifier<AccountId> {
    /// Returns `true` if `doctor` has access to `patient`'s data.
    fn has_access(patient: &AccountId, doctor: &AccountId) -> bool;
}

impl<T: pallet::Config> MedicalPermissionsVerifier<T::AccountId> for pallet::Pallet<T> {
    fn has_access(patient: &T::AccountId, doctor: &T::AccountId) -> bool {
        // A patient always has access to their own data.
        if patient == doctor {
            return true;
        }

        pallet::Permissions::<T>::get(patient, doctor)
    }
}
