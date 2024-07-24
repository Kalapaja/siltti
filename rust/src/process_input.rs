//! Basic processing of QR inputs.
use frame_metadata::v15::RuntimeMetadataV15;
use lt_codes::mock_worst_case::Encoder;
use metadata_shortener::{cutter::cut_metadata, traits::Blake3Leaf, MetadataDescriptor};
use parity_scale_codec::Encode;
use primitive_types::H256;
use std::{
    convert::TryInto,
    sync::{Arc, RwLock},
};
use substrate_crypto_light::sr25519::{Public, PUBLIC_LEN};
use substrate_parser::{error::SignableError, parse_transaction};

use crate::database::ValueMetadataSpecs;
use crate::definitions::{Bytes, DerivationInfo, MetadataSet, Transaction, TransmittableContent};
use crate::error::ErrorCompanion;
use crate::sign_with_companion::{SignByCompanion, SignatureMaker};

pub const PREFIX_SUBSTRATE: u8 = 0x53;

pub const ID_SIGNABLE: &[u8] = &[0x00, 0x02];
pub const ID_BYTES: u8 = 0x03;
pub const ENCRYPTION_SR25519: u8 = 0x01;

impl Transaction {
    pub fn from_payload_prelude_cut(
        mut payload: &[u8],
        db_path: &str,
    ) -> Result<Self, ErrorCompanion> {
        let signer = match payload.get(0..PUBLIC_LEN) {
            Some(public_key_slice) => {
                payload = &payload[PUBLIC_LEN..];
                Public(public_key_slice.try_into().expect("stable known length"))
            }
            None => return Err(ErrorCompanion::TooShort),
        };
        if payload.len() >= H256::len_bytes() {
            let genesis_hash = H256(
                payload[payload.len() - H256::len_bytes()..]
                    .try_into()
                    .expect("stable known length"),
            );
            let metadata_specs = match ValueMetadataSpecs::try_get_db(genesis_hash, db_path)? {
                Some(a) => a.inner(),
                None => return Err(ErrorCompanion::LoadSpecsMetadata { genesis_hash }),
            };
            let signable_transaction = payload[..payload.len() - H256::len_bytes()].to_vec();

            // check that decoding is possible with complete metadata
            match parse_transaction::<&[u8], (), RuntimeMetadataV15>(
                &signable_transaction.as_ref(),
                &mut (),
                &metadata_specs.metadata,
                Some(genesis_hash),
            ) {
                Ok(_) => {
                    let short_metadata = cut_metadata::<&[u8], (), Blake3Leaf, RuntimeMetadataV15>(
                        &signable_transaction.as_ref(),
                        &mut (),
                        &metadata_specs.metadata,
                        &metadata_specs.specs,
                    )
                    .map_err(ErrorCompanion::MetaCut)?;
                    match short_metadata.metadata_descriptor {
                        MetadataDescriptor::V1 {
                            call_ty,
                            signed_extensions,
                            spec_name_version,
                            base58prefix,
                            decimals,
                            unit,
                        } => Ok(Self {
                            genesis_hash,
                            encoded_metadata_set: MetadataSet {
                                types: short_metadata.short_registry,
                                call_ty,
                                signed_extensions,
                                spec_name_version,
                                base58prefix,
                                decimals,
                                unit,
                            }
                            .encode(),
                            encoded_signable_transaction: signable_transaction.encode(),
                            signer,
                        }),
                        _ => unreachable!(),
                    }
                }
                Err(SignableError::WrongSpecVersion {
                    as_decoded,
                    in_metadata,
                }) => Err(ErrorCompanion::UpdateMetadata {
                    as_decoded,
                    in_metadata,
                }),
                Err(e) => Err(ErrorCompanion::TransactionNotParsable(e)),
            }
        } else {
            Err(ErrorCompanion::TooShort)
        }
    }
}

impl Bytes {
    pub fn from_payload_prelude_cut(payload: &[u8]) -> Result<Self, ErrorCompanion> {
        match payload.get(0..PUBLIC_LEN) {
            Some(public_key_slice) => {
                let bytes_uncut = payload[PUBLIC_LEN..].to_vec();
                let signer = Public(public_key_slice.try_into().expect("stable known length"));
                Ok(Self {
                    bytes_uncut,
                    signer,
                })
            }
            None => Err(ErrorCompanion::TooShort),
        }
    }
}

#[derive(Debug, uniffi::Object)]
pub enum Action {
    Success,
    Transmit(Transmit),
}

#[derive(Debug)]
pub struct Transmit {
    data_with_signature: Vec<u8>,
    encoder: RwLock<Encoder>,
}

#[derive(Debug)]
pub struct Transmittable {
    content: TransmittableContent,
    signature_maker: Arc<dyn SignByCompanion>,
}

impl Transmittable {
    pub fn into_transmit(self) -> Result<Transmit, ErrorCompanion> {
        let encoded_data = self.content.encode();
        let signature_maker = SignatureMaker::new(self.signature_maker);
        let data_with_signature = signature_maker.signed_data(encoded_data);
        let encoder = Encoder::init(&data_with_signature).map_err(|_| ErrorCompanion::LTError)?;
        Ok(Transmit {
            data_with_signature,
            encoder: RwLock::new(encoder),
        })
    }
}

#[uniffi::export]
impl Action {
    #[uniffi::constructor(name = "new_kampela_stop")]
    pub fn new_kampela_stop(
        signature_maker: Arc<dyn SignByCompanion>,
    ) -> Result<Self, ErrorCompanion> {
        let transmittable = Transmittable {
            content: TransmittableContent::KampelaStop,
            signature_maker,
        };
        Ok(Self::Transmit(transmittable.into_transmit()?))
    }

    #[uniffi::constructor(name = "new_payload")]
    pub fn new_payload(
        mut payload: &[u8],
        db_path: &str,
        signature_maker: Arc<dyn SignByCompanion>,
    ) -> Result<Self, ErrorCompanion> {
        match payload.get(..3) {
            Some(prelude) => {
                payload = &payload[3..];
                if prelude[0] != PREFIX_SUBSTRATE {
                    return Err(ErrorCompanion::NotSubstrate);
                }
                if prelude[1] != ENCRYPTION_SR25519 {
                    return Err(ErrorCompanion::OnlySr25519(prelude[1]));
                }
                match prelude[2] {
                    a if ID_SIGNABLE.contains(&a) => {
                        let transaction = Transaction::from_payload_prelude_cut(payload, db_path)?;
                        let transmittable = Transmittable {
                            content: TransmittableContent::SignableTransaction(transaction),
                            signature_maker,
                        };
                        Ok(Self::Transmit(transmittable.into_transmit()?))
                    }
                    ID_BYTES => {
                        let bytes = Bytes::from_payload_prelude_cut(payload)?;
                        let transmittable = Transmittable {
                            content: TransmittableContent::Bytes(bytes),
                            signature_maker,
                        };
                        Ok(Self::Transmit(transmittable.into_transmit()?))
                    }
                    a => Err(ErrorCompanion::UnknownPayloadType(a)),
                }
            }
            None => Err(ErrorCompanion::TooShort),
        }
    }

    #[uniffi::constructor(name = "new_derivation")]
    pub fn new_derivation(
        cut_path: String,
        has_pwd: bool,
        signature_maker: Arc<dyn SignByCompanion>,
    ) -> Result<Self, ErrorCompanion> {
        let derivation = DerivationInfo { cut_path, has_pwd };
        let transmittable = Transmittable {
            content: TransmittableContent::Derivation(derivation),
            signature_maker,
        };
        Ok(Self::Transmit(transmittable.into_transmit()?))
    }

    #[uniffi::method(name = "is_transmit")]
    pub fn is_transmit(&self) -> bool {
        matches!(self, Action::Transmit(_))
    }

    #[uniffi::method(name = "make_packet")]
    pub fn make_packet(self: &Arc<Self>) -> Option<Vec<u8>> {
        match self.as_ref() {
            Action::Success => None,
            Action::Transmit(transmit) => {
                let mut encoder = match transmit.encoder.write() {
                    Ok(a) => a,
                    Err(_) => return None,
                };
                if let Ok(Some(packet)) = encoder.make_packet(&transmit.data_with_signature) {
                    Some(packet.serialize().to_vec())
                } else {
                    None
                }
            }
        }
    }
}
