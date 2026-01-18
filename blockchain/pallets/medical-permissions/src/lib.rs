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

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Pallet configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>>
            + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Weight information for extrinsics.
        type WeightInfo: WeightInfo;
    }

    // ---------------------------------------------------------------------
    // Storage
    // ---------------------------------------------------------------------

    /// Access permissions mapping: `(patient, doctor) -> has_access`.
    ///
    /// If the key is missing, the value defaults to `false` due to `ValueQuery`.
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
        /// Access was granted from a patient to a doctor.
        AccessGranted { patient: T::AccountId, doctor: T::AccountId },

        /// Access was revoked from a patient to a doctor.
        AccessRevoked { patient: T::AccountId, doctor: T::AccountId },
    }

    // ---------------------------------------------------------------------
    // Errors
    // ---------------------------------------------------------------------

    #[pallet::error]
    pub enum Error<T> {
        /// A patient cannot grant permission to themselves.
        SelfPermissionNotAllowed,
    }

    // ---------------------------------------------------------------------
    // Calls (extrinsics)
    // ---------------------------------------------------------------------

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Grant access to a doctor.
        ///
        /// # Parameters
        /// - `origin`: Must be a signed account (patient).
        /// - `doctor`: The doctor account that will receive access.
        ///
        /// # Storage
        /// - Writes: [`Permissions`]
        ///
        /// # Emits
        /// - [`Event::AccessGranted`]
        ///
        /// # Errors
        /// - [`Error::SelfPermissionNotAllowed`]: If `patient == doctor`.
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn grant_access(origin: OriginFor<T>, doctor: T::AccountId) -> DispatchResult {
            let patient = ensure_signed(origin)?;

            ensure!(
                patient != doctor,
                Error::<T>::SelfPermissionNotAllowed
            );

            Permissions::<T>::insert(&patient, &doctor, true);
            Self::deposit_event(Event::AccessGranted { patient, doctor });

            Ok(())
        }

        /// Revoke access from a doctor.
        ///
        /// # Parameters
        /// - `origin`: Must be a signed account (patient).
        /// - `doctor`: The doctor account that will lose access.
        ///
        /// # Storage
        /// - Writes: [`Permissions`]
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

/// Public verifier interface for checking whether a doctor has access to a
/// patient's medical data.
///
/// Other pallets can depend on this trait to enforce authorization.
pub trait MedicalPermissionsVerifier<AccountId> {
    /// Returns `true` if `doctor` has access to `patient`'s data.
    fn has_access(patient: &AccountId, doctor: &AccountId) -> bool;
}

impl<T: pallet::Config> MedicalPermissionsVerifier<T::AccountId> for pallet::Pallet<T> {
    fn has_access(patient: &T::AccountId, doctor: &T::AccountId) -> bool {
        if patient == doctor {
            return true;
        }

        pallet::Permissions::<T>::get(patient, doctor)
    }
}
