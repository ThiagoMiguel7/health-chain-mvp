use codec::{Decode, Encode, MaxEncodedLen};
// CORREÇÃO: RuntimeDebug vem de pallet_prelude, não da raiz
use frame_support::{BoundedVec, pallet_prelude::RuntimeDebug, traits::ConstU32};
use scale_info::TypeInfo;

/// Hash of a medical file (fixed-length 64 bytes).
/// 
/// Defined in types.rs to keep lib.rs clean.
pub type FileHash = BoundedVec<u8, ConstU32<64>>;

/// Represents a medical record reference stored on-chain.
///
/// This struct stores metadata about a medical file hash:
/// - who created it (`created_by`)
/// - when it was created (`created_at`)
/// - the file hash itself (`file_hash`)
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct MedicalRecord<AccountId, Moment> {
    /// The account that created the record (doctor).
    pub created_by: AccountId,
    /// Timestamp when the record was created.
    pub created_at: Moment,
    /// File hash reference.
    pub file_hash: FileHash,
}