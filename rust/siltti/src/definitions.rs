use metadata_shortener::ShortRegistry;
use parity_scale_codec::{Decode, Encode};
use primitive_types::H256;
use scale_info::{interner::UntrackedSymbol};
use std::any::TypeId;
use substrate_crypto_light::sr25519::Public;
use substrate_parser::traits::{SignedExtensionMetadata, SpecNameVersion};

#[derive(Debug, Decode, Encode)]
#[repr(C)]
pub struct TransferData {
    pub encoded_data: Vec<u8>,
    pub companion_signature: Vec<u8>,
    pub companion_public_key: Vec<u8>,
}

#[derive(Debug, Decode, Encode, Eq, PartialEq)]
pub enum TransmittableContent {
    #[codec(index = 0)]
    KampelaStop,

    #[codec(index = 1)]
    Bytes(Bytes),

    #[codec(index = 2)]
    Derivation(DerivationInfo),

    #[codec(index = 3)]
    SignableTransaction(Transaction),
}

#[derive(Debug, Decode, Encode, Eq, PartialEq)]
#[repr(C)]
pub struct Bytes {
    pub bytes_uncut: Vec<u8>,
    pub signer: Public,
}

#[derive(Debug, Decode, Encode, Eq, PartialEq)]
#[repr(C)]
pub struct Transaction {
    pub genesis_hash: H256,
    pub encoded_metadata_set: Vec<u8>,
    pub encoded_signable_transaction: Vec<u8>,
    pub signer: Public,
}

#[derive(Debug, Decode, Encode, Eq, PartialEq)]
#[repr(C)]
pub struct MetadataSet {
    pub types: ShortRegistry,
    pub call_ty: UntrackedSymbol<TypeId>,
    pub signed_extensions: Vec<SignedExtensionMetadata>,
    pub spec_name_version: SpecNameVersion,
    pub base58prefix: u16,
    pub decimals: u8,
    pub unit: String,
}

#[derive(Debug, Decode, Encode, Eq, PartialEq)]
#[repr(C)]
pub struct DerivationInfo {
    pub cut_path: String,
    pub has_pwd: bool,
}
