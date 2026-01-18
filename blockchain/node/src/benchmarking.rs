//! Setup code for [`super::command`] which would otherwise bloat that module.
//!
//! Should only be used for benchmarking as it may break in other contexts.

use crate::service::FullClient;

use healthchain_runtime::{
    AccountId, BalancesCall, Runtime, RuntimeCall, SystemCall, UncheckedExtrinsic, VERSION,
};
use sc_cli::Result;
use sc_client_api::UsageProvider;
use sp_core::{Encode, Pair};
use sp_inherents::{InherentData, InherentDataProvider};
use sp_keyring::Sr25519Keyring;
use sp_runtime::{OpaqueExtrinsic, SaturatedConversion};
use std::{sync::Arc, time::Duration};

/// Generates extrinsics for the `benchmark overhead` command.
///
/// Note: Should only be used for benchmarking.
pub struct RemarkBuilder {
    client: Arc<FullClient>,
}

impl RemarkBuilder {
    /// Creates a new [`RemarkBuilder`].
    pub fn new(client: Arc<FullClient>) -> Self {
        Self { client }
    }
}

impl frame_benchmarking_cli::ExtrinsicBuilder for RemarkBuilder {
    fn pallet(&self) -> &str {
        "system"
    }

    fn extrinsic(&self) -> &str {
        "remark"
    }

    fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
        let acc = Sr25519Keyring::Bob.pair();
        let extrinsic: UncheckedExtrinsic = create_benchmark_extrinsic(
            self.client.as_ref(),
            acc,
            SystemCall::remark { remark: vec![] }.into(),
            nonce,
        )
        .map_err(|_| "Failed to create extrinsic")?;

        Ok(extrinsic.into())
    }
}

/// Generates `Create` and `Transfer` extrinsics for the `benchmark overhead` command.
///
/// Note: Should only be used for benchmarking.
pub struct TransferKeepAliveBuilder {
    client: Arc<FullClient>,
    dest: AccountId,
    value: u128,
}

impl TransferKeepAliveBuilder {
    /// Creates a new [`TransferKeepAliveBuilder`].
    pub fn new(client: Arc<FullClient>, dest: AccountId, value: u128) -> Self {
        Self {
            client,
            dest,
            value,
        }
    }
}

impl frame_benchmarking_cli::ExtrinsicBuilder for TransferKeepAliveBuilder {
    fn pallet(&self) -> &str {
        "balances"
    }

    fn extrinsic(&self) -> &str {
        "transfer_keep_alive"
    }

    fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
        let acc = Sr25519Keyring::Alice.pair();
        let extrinsic: UncheckedExtrinsic = create_benchmark_extrinsic(
            self.client.as_ref(),
            acc,
            BalancesCall::transfer_keep_alive {
                dest: sp_runtime::MultiAddress::Id(self.dest.clone()),
                value: self.value,
            }
            .into(),
            nonce,
        )
        .map_err(|_| "Failed to create extrinsic")?;

        Ok(extrinsic.into())
    }
}

/// Create a transaction using the given `call`.
///
/// Note: Should only be used for benchmarking.
pub fn create_benchmark_extrinsic(
    client: &FullClient,
    sender: sp_core::sr25519::Pair,
    call: RuntimeCall,
    nonce: u32,
) -> Result<UncheckedExtrinsic> {
    let genesis_hash = client.usage_info().chain.genesis_hash;

    // CORREÇÃO AQUI: Acessamos best_hash e best_number diretamente
    let best_hash = client.usage_info().chain.best_hash;
    let best_block = client.usage_info().chain.best_number;

    let period = 2400u64;

    let extra = (
        frame_system::AuthorizeCall::<Runtime>::new(),
        frame_system::CheckNonZeroSender::<Runtime>::new(),
        frame_system::CheckSpecVersion::<Runtime>::new(),
        frame_system::CheckTxVersion::<Runtime>::new(),
        frame_system::CheckGenesis::<Runtime>::new(),
        frame_system::CheckEra::<Runtime>::from(sp_runtime::generic::Era::mortal(
            period,
            best_block.saturated_into(),
        )),
        frame_system::CheckNonce::<Runtime>::from(nonce),
        frame_system::CheckWeight::<Runtime>::new(),
        pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(0),
        frame_metadata_hash_extension::CheckMetadataHash::<Runtime>::new(false),
        frame_system::WeightReclaim::<Runtime>::new(),
    );

    let raw_payload = sp_runtime::generic::SignedPayload::from_raw(
        call.clone(),
        extra.clone(),
        (
            (), // AuthorizeCall
            (), // CheckNonZeroSender
            VERSION.spec_version,
            VERSION.transaction_version,
            genesis_hash,
            best_hash, // CheckEra
            (),        // CheckNonce
            (),        // CheckWeight
            (),        // ChargeTransactionPayment
            None,      // CheckMetadataHash
            (),        // WeightReclaim
        ),
    );
    let signature = raw_payload.using_encoded(|e| sender.sign(e));

    Ok(UncheckedExtrinsic::new_signed(
        call,
        sp_runtime::MultiAddress::Id(sender.public().into()),
        sp_runtime::MultiSignature::Sr25519(signature),
        extra,
    ))
}

/// Generates inherent data for the `benchmark overhead` command.
///
/// Note: Should only be used for benchmarking.
pub fn inherent_benchmark_data() -> Result<InherentData> {
    let mut inherent_data = InherentData::new();
    let d = Duration::from_millis(0);
    let timestamp = sp_timestamp::InherentDataProvider::new(d.into());

    futures::executor::block_on(timestamp.provide_inherent_data(&mut inherent_data))
        .map_err(|e| format!("creating inherent data: {:?}", e))?;
    Ok(inherent_data)
}
