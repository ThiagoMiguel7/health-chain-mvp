#![cfg_attr(not(feature = "std"), no_std)]

//! # HealthChain Runtime
//!
//! Substrate runtime configuration for the HealthChain MVP.
//!
//! This file wires together FRAME pallets, defines core runtime types, and
//! provides pallet `Config` implementations plus genesis presets.

#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

extern crate alloc;

use alloc::vec::Vec;

use frame_support::{
    construct_runtime, derive_impl,
    traits::{ConstBool, ConstU128, ConstU32, ConstU64, ConstU8, Get},
    weights::IdentityFee,
};
use sp_runtime::{
    generic, impl_opaque_keys,
    traits::{BlakeTwo256, IdentifyAccount, Verify},
    MultiAddress, MultiSignature,
};
use sp_version::RuntimeVersion;

// Re-exports commonly used by downstream crates / RPC layers.
pub use frame_system::Call as SystemCall;
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;

#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;

/// Opaque types used by the CLI to interact with the runtime.
pub mod opaque {
    use super::*;
    use sp_runtime::traits::Hash as HashT;

    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    pub type BlockId = generic::BlockId<Block>;
    pub type Hash = <BlakeTwo256 as HashT>::Output;
}

impl_opaque_keys! {
    /// Session keys used by the consensus engine.
    pub struct SessionKeys {
        pub aura: Aura,
        pub grandpa: Grandpa,
    }
}

#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: alloc::borrow::Cow::Borrowed("healthchain-runtime"),
    impl_name: alloc::borrow::Cow::Borrowed("healthchain-runtime"),
    authoring_version: 1,
    spec_version: 101,
    impl_version: 1,
    apis: apis::RUNTIME_API_VERSIONS,
    transaction_version: 1,
    system_version: 1,
};

pub struct Version;

impl Get<RuntimeVersion> for Version {
    fn get() -> RuntimeVersion {
        VERSION
    }
}

// ----------------------------------------------------------------------------
// Constants
// ----------------------------------------------------------------------------

pub const MILLI_SECS_PER_BLOCK: u64 = 6_000;
pub const SLOT_DURATION: u64 = MILLI_SECS_PER_BLOCK;

pub const UNIT: Balance = 1_000_000_000_000;
pub const MILLI_UNIT: Balance = 1_000_000_000;
pub const MICRO_UNIT: Balance = 1_000_000;
pub const EXISTENTIAL_DEPOSIT: Balance = MILLI_UNIT;

// ----------------------------------------------------------------------------
// Primitive Types
// ----------------------------------------------------------------------------

pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Balance = u128;
pub type Nonce = u32;
pub type Hash = sp_core::H256;
pub type BlockNumber = u32;
pub type Address = MultiAddress<AccountId, ()>;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
pub type SignedBlock = generic::SignedBlock<Block>;
pub type BlockId = generic::BlockId<Block>;

// ----------------------------------------------------------------------------
// Runtime Construction
// ----------------------------------------------------------------------------

construct_runtime!(
    pub enum Runtime {
        System: frame_system,
        Timestamp: pallet_timestamp,
        Aura: pallet_aura,
        Grandpa: pallet_grandpa,
        Balances: pallet_balances,
        TransactionPayment: pallet_transaction_payment,
        Sudo: pallet_sudo,

        // --- HealthChain pallets ---
        MedicalHistory: pallet_medical_history,
        MedicalPermissions: pallet_medical_permissions,
        MedicalHistoryReader: pallet_medical_history_reader,
    }
);

pub type TxExtension = (
    frame_system::AuthorizeCall<Runtime>,
    frame_system::CheckNonZeroSender<Runtime>,
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
    frame_metadata_hash_extension::CheckMetadataHash<Runtime>,
    frame_system::WeightReclaim<Runtime>,
);

pub type UncheckedExtrinsic =
    generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, TxExtension>;

pub type SignedPayload = generic::SignedPayload<RuntimeCall, TxExtension>;

pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;

// ----------------------------------------------------------------------------
// Pallet Configs
// ----------------------------------------------------------------------------

#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig)]
impl frame_system::Config for Runtime {
    type Block = Block;
    type BlockHashCount = ConstU32<2400>;
    type AccountData = pallet_balances::AccountData<Balance>;
    type Version = Version;
}

impl pallet_aura::Config for Runtime {
    type AuthorityId = sp_consensus_aura::sr25519::AuthorityId;
    type DisabledValidators = ();
    type MaxAuthorities = ConstU32<32>;
    type AllowMultipleBlocksPerSlot = ConstBool<false>;
    type SlotDuration = pallet_aura::MinimumPeriodTimesTwo<Runtime>;
}

impl pallet_grandpa::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MaxAuthorities = ConstU32<32>;
    type MaxNominators = ConstU32<0>;
    type MaxSetIdSessionEntries = ConstU64<0>;
    type KeyOwnerProof = sp_core::Void;
    type EquivocationReportSystem = ();
}

impl pallet_timestamp::Config for Runtime {
    type Moment = u64;
    type OnTimestampSet = Aura;
    type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
    type WeightInfo = ();
}

impl pallet_balances::Config for Runtime {
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = Balance;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
    type AccountStore = System;
    type WeightInfo = ();
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = RuntimeHoldReason;
    type RuntimeFreezeReason = RuntimeFreezeReason;
    type DoneSlashHandler = ();
}

impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeTransaction = pallet_transaction_payment::FungibleAdapter<Balances, ()>;
    type OperationalFeeMultiplier = ConstU8<5>;
    type WeightToFee = IdentityFee<Balance>;
    type LengthToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = ();
    type WeightInfo = ();
}

impl pallet_sudo::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type WeightInfo = ();
}

// ----------------------------------------------------------------------------
// HealthChain Pallets
// ----------------------------------------------------------------------------

impl pallet_medical_history::Config for Runtime {
    type WeightInfo = ();
    type Permissions = MedicalPermissions;
}

impl pallet_medical_permissions::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

impl pallet_medical_history_reader::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type HistoryProvider = MedicalHistory;
    type Permissions = MedicalPermissions;
}

// ----------------------------------------------------------------------------
// Genesis Presets
// ----------------------------------------------------------------------------

pub mod genesis_config_presets {
    use super::*;
    use alloc::{string::String, vec, vec::Vec};
    use sp_genesis_builder::PresetId;
    use sp_keyring::Sr25519Keyring;

    pub fn preset_names() -> Vec<String> {
        vec![String::from("dev"), String::from("local")]
    }

    pub fn get_preset(id: &PresetId) -> Option<Vec<u8>> {
        let patch = match id.as_ref() {
            sp_genesis_builder::DEV_RUNTIME_PRESET => serde_json::json!({
                "balances": {
                    "balances": vec![
                        (Sr25519Keyring::Alice.to_account_id(), 1u64 << 60),
                        (Sr25519Keyring::Bob.to_account_id(), 1u64 << 60),
                    ],
                },
                "sudo": { "key": Sr25519Keyring::Alice.to_account_id() },
                "aura": { "authorities": vec![Sr25519Keyring::Alice.public()] },
                "grandpa": { "authorities": vec![(Sr25519Keyring::Alice.public(), 1)] },
            }),
            _ => return None,
        };

        Some(serde_json::to_vec(&patch).expect("genesis preset patch serialization must succeed"))
    }
}

pub mod apis;
