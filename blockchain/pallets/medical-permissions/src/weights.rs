#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

// ==========================================================================
// ⚠️  PLACEHOLDER / MOCK WEIGHTS
// 
// TODO: This file is currently manually maintained for MVP development.
// Before Mainnet release, replace this entire file with the output of:
// ./target/release/healthchain-node benchmark pallet ...
// ==========================================================================

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

pub trait WeightInfo {
    fn grant_access() -> Weight { Weight::from_parts(10_000, 0) }
    fn revoke_access() -> Weight { Weight::from_parts(10_000, 0) }
}

/// Weights for pallet_medical_permissions using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn grant_access() -> Weight {
        Weight::from_parts(10_000, 0)
    }
    fn revoke_access() -> Weight {
        Weight::from_parts(10_000, 0)
    }
}

// For backwards compatibility and tests.
impl WeightInfo for () {
    fn grant_access() -> Weight {
        Weight::from_parts(10_000, 0)
    }
    fn revoke_access() -> Weight {
        Weight::from_parts(10_000, 0)
    }
}