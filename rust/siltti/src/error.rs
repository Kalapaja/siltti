//! Errors occuring in companion.
use frame_metadata::v15::RuntimeMetadataV15;
use jsonrpsee::core::ClientError;
use metadata_shortener::error::MetaCutError;
use primitive_types::H256;
use substrate_parser::error::SignableError;

#[derive(Debug, thiserror::Error, uniffi::Error)]
#[uniffi(flat_error)]
pub enum ErrorCompanion {
    #[error("Format of fetched base58 prefix {value} is not supported.")]
    Base58PrefixFormatNotSupported { value: String },

    #[error("Base58 prefixes in metadata {meta} and specs {specs} do not match.")]
    Base58PrefixMismatch { specs: u16, meta: u16 },

    #[error("Unexpected block hash format.")]
    BlockHashFormat,

    #[error("Ws client error. {0}")]
    Client(ClientError),

    #[error("Internal database error. {0}")]
    DbInternal(sled::Error),

    #[error("Database error recording transaction. {0}")]
    DbTransaction(sled::transaction::TransactionError),

    #[error("Format of fetched decimals {value} is not supported.")]
    DecimalsFormatNotSupported { value: String },

    #[error("Fetch address in the database for genesis hash {} got damaged, and could not be decoded.", hex::encode(.0))]
    DecodeDbAddress(H256),

    #[error("Metadata in the database for genesis hash {} got damaged, and could not be decoded.", hex::encode(.0))]
    DecodeDbMetadata(H256),

    #[error("Specs in the database for genesis hash {} got damaged, and could not be decoded.", hex::encode(.0))]
    DecodeDbSpecs(H256),

    #[error("Unexpected genesis hash format.")]
    GenesisHashFormat,

    #[error("Unexpected genesis hash length.")]
    GenesisHashLength,

    #[error("No metadata for chain with genesis hash {} in the database.", hex::encode(genesis_hash))]
    LoadMetadata{genesis_hash: H256},

    #[error("No specs for chain with genesis hash {} in the database.", hex::encode(genesis_hash))]
    LoadSpecs{genesis_hash: H256},

    #[error("Error generating LT codes.")]
    LTError,

    #[error("Error cutting metadata for signable transaction. {0}")]
    MetaCut(MetaCutError<(), RuntimeMetadataV15>),

    #[error("...")]
    MetadataFormat,

    #[error("...")]
    MetadataNotDecodeable,

    #[error("No base58 prefix is fetched as system properties or found in the metadata.")]
    NoBase58Prefix,

    #[error("No decimals value is fetched.")]
    NoDecimals,

    #[error("Metadata v15 not available through rpc.")]
    NoMetadataV15,

    #[error("Metadata must start with `meta` prefix.")]
    NoMetaPrefix,

    #[error("{0}")]
    NotHex(NotHex),

    #[error("Fetched values were not sent through successfully.")]
    NotSent,

    #[error("Received QR payload is not a Substrate one.")]
    NotSubstrate,

    #[error("No unit value is fetched.")]
    NoUnit,

    #[error("Only Sr25519 encryption, 0x01, is supported. Received transaction has encoded encryption 0x{}", hex::encode([*.0]))]
    OnlySr25519(u8),

    #[error("...")]
    PropertiesFormat,

    #[error("...")]
    RawMetadataNotDecodeable,

    #[error("Can't read data through the interface. Receiver closed.")]
    ReceiverClosed,

    #[error("Can't read data through the interface. Receiver guard is poisoned.")]
    ReceiverGuardPoisoned,

    #[error("Received QR payload is too short.")]
    TooShort,

    #[error("Received transaction could not be parsed. {0}.")]
    TransactionNotParsable(SignableError<(), RuntimeMetadataV15>),

    #[error("Unexpected payload type, 0x{}", hex::encode([*.0]))]
    UnknownPayloadType(u8),

    #[error("Format of fetched unit {value} is not supported.")]
    UnitFormatNotSupported { value: String },

    #[error("Try updating metadata. Metadata version in transaction {as_decoded} does not match the version of the available metadata entry {in_metadata}.")]
    UpdateMetadata{as_decoded: String, in_metadata: String}
}

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum NotHex {
    #[error("Genesis hash string is not a valid hexadecimal.")]
    GenesisHash,

    #[error("Encoded metadata string is not a valid hexadecimal.")]
    Metadata,
}
