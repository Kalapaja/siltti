//! Basic processing of QR inputs.

use lt_codes::mock_worst_case::Encoder;
use parity_scale_codec::{Decode, Encode};
use sp_core::H256;
use std::{
    convert::TryInto,
    sync::{Arc, RwLock},
};
use substrate_parser::cut_metadata::cut_metadata;

use kampela_common::{
    Bytes, DerivationInfo, Encryption, MultiSigner, Transaction, TransmittableContent,
};

use crate::error::ErrorCompanion;
use crate::sign_with_companion::{SignByCompanion, SignatureMaker};
use crate::storage::{MetadataStorage, SpecsStorage};

pub const PREFIX_SUBSTRATE: u8 = 0x53;

pub const ID_SIGNABLE: &[u8] = &[0x00, 0x02];
pub const ID_BYTES: u8 = 0x03;
pub const ID_METADATA: u8 = 0x80;
pub const ID_SPECS: u8 = 0xc1;

pub trait FromQrAndDb: Sized {
    fn from_payload_prelude_cut(
        payload: &[u8],
        encryption: &Encryption,
        db_path: &str,
    ) -> Result<Self, ErrorCompanion>;
}

impl FromQrAndDb for Transaction {
    fn from_payload_prelude_cut(
        mut payload: &[u8],
        encryption: &Encryption,
        db_path: &str,
    ) -> Result<Self, ErrorCompanion> {
        let signer = match payload.get(0..encryption.key_length()) {
            Some(public_key_slice) => {
                payload = &payload[encryption.key_length()..];
                match encryption {
                    Encryption::Ed25519 => MultiSigner::Ed25519(
                        public_key_slice.try_into().expect("stable known length"),
                    ),
                    Encryption::Sr25519 => MultiSigner::Sr25519(
                        public_key_slice.try_into().expect("stable known length"),
                    ),
                    Encryption::Ecdsa => MultiSigner::Ecdsa(
                        public_key_slice.try_into().expect("stable known length"),
                    ),
                }
            }
            None => return Err(ErrorCompanion::TooShort),
        };
        if payload.len() >= H256::len_bytes() {
            let genesis_hash = H256(
                payload[payload.len() - H256::len_bytes()..]
                    .try_into()
                    .expect("stable known length"),
            );
            let metadata_storage = MetadataStorage::read_from_db(db_path, genesis_hash)?;
            let specs_storage = SpecsStorage::read_from_db(db_path, *encryption, genesis_hash)?;
            let signable_transaction = payload[..payload.len() - H256::len_bytes()].to_vec();
            let short_metadata = cut_metadata(
                &signable_transaction.as_ref(),
                &mut (),
                &metadata_storage.value,
                &specs_storage.value.short_specs,
            )
            .map_err(ErrorCompanion::MetaCut)?;
            Ok(Self {
                genesis_hash,
                encoded_short_meta: short_metadata.encode(),
                encoded_signable_transaction: signable_transaction.encode(),
                signer,
            })
        } else {
            Err(ErrorCompanion::TooShort)
        }
    }
}

pub trait FromQr: Sized {
    fn from_payload_prelude_cut(
        payload: &[u8],
        encryption: &Encryption,
    ) -> Result<Self, ErrorCompanion>;
}

impl FromQr for Bytes {
    fn from_payload_prelude_cut(
        payload: &[u8],
        encryption: &Encryption,
    ) -> Result<Self, ErrorCompanion> {
        match payload.get(0..encryption.key_length()) {
            Some(public_key_slice) => {
                let bytes_uncut = payload[encryption.key_length()..].to_vec();
                let signer = match encryption {
                    Encryption::Ed25519 => MultiSigner::Ed25519(
                        public_key_slice.try_into().expect("stable known length"),
                    ),
                    Encryption::Sr25519 => MultiSigner::Sr25519(
                        public_key_slice.try_into().expect("stable known length"),
                    ),
                    Encryption::Ecdsa => MultiSigner::Ecdsa(
                        public_key_slice.try_into().expect("stable known length"),
                    ),
                };
                Ok(Self {
                    bytes_uncut,
                    signer,
                })
            }
            None => Err(ErrorCompanion::TooShort),
        }
    }
}

#[derive(Debug)]
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
    signature_maker: Box<dyn SignByCompanion>,
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

impl Action {
    pub fn new_kampela_stop(
        signature_maker: Box<dyn SignByCompanion>,
    ) -> Result<Self, ErrorCompanion> {
        let transmittable = Transmittable {
            content: TransmittableContent::KampelaStop,
            signature_maker,
        };
        Ok(Self::Transmit(transmittable.into_transmit()?))
    }

    pub fn new_payload(
        mut payload: &[u8],
        db_path: &str,
        signature_maker: Box<dyn SignByCompanion>,
    ) -> Result<Self, ErrorCompanion> {
        match payload.get(..3) {
            Some(prelude) => {
                payload = &payload[3..];
                if prelude[0] != PREFIX_SUBSTRATE {
                    return Err(ErrorCompanion::NotSubstrate);
                }
                match prelude[2] {
                    a if ID_SIGNABLE.contains(&a) => {
                        let encryption = Encryption::decode(&mut &prelude[1..2])
                            .map_err(|_| ErrorCompanion::UnknownSigningAlgorithm(prelude[1]))?;
                        let transaction =
                            Transaction::from_payload_prelude_cut(payload, &encryption, db_path)?;
                        let transmittable = Transmittable {
                            content: TransmittableContent::SignableTransaction(transaction),
                            signature_maker,
                        };
                        Ok(Self::Transmit(transmittable.into_transmit()?))
                    }
                    ID_BYTES => {
                        let encryption = Encryption::decode(&mut &prelude[1..2])
                            .map_err(|_| ErrorCompanion::UnknownSigningAlgorithm(prelude[1]))?;
                        let bytes = Bytes::from_payload_prelude_cut(payload, &encryption)?;
                        let transmittable = Transmittable {
                            content: TransmittableContent::Bytes(bytes),
                            signature_maker,
                        };
                        Ok(Self::Transmit(transmittable.into_transmit()?))
                    }
                    ID_METADATA => {
                        let encryption = Encryption::decode(&mut &prelude[1..2])
                            .map_err(|_| ErrorCompanion::UnknownSigningAlgorithm(prelude[1]))?;
                        let metadata_storage =
                            MetadataStorage::from_payload_prelude_cut(payload, &encryption)?;
                        metadata_storage.write_in_db(db_path)?;
                        Ok(Self::Success)
                    }
                    ID_SPECS => {
                        let encryption = Encryption::decode(&mut &prelude[1..2])
                            .map_err(|_| ErrorCompanion::UnknownSigningAlgorithm(prelude[1]))?;
                        let specs_storage =
                            SpecsStorage::from_payload_prelude_cut(payload, &encryption)?;
                        specs_storage.write_in_db(db_path)?;
                        Ok(Self::Success)
                    }
                    a => Err(ErrorCompanion::UnknownPayloadType(a)),
                }
            }
            None => Err(ErrorCompanion::TooShort),
        }
    }

    pub fn new_derivation(
        cut_path: String,
        has_pwd: bool,
        signature_maker: Box<dyn SignByCompanion>,
    ) -> Result<Self, ErrorCompanion> {
        let derivation = DerivationInfo { cut_path, has_pwd };
        let transmittable = Transmittable {
            content: TransmittableContent::Derivation(derivation),
            signature_maker,
        };
        Ok(Self::Transmit(transmittable.into_transmit()?))
    }

    pub fn is_transmit(&self) -> bool {
        if let Action::Transmit(_) = self { true } else { false }
    }

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
