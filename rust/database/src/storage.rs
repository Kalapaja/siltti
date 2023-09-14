//! Keys and corresponding values in companion database.
use frame_metadata::{v14::RuntimeMetadataV14, RuntimeMetadata, META_RESERVED};
use parity_scale_codec::{Decode, Encode};
use sled::{open, Db, IVec, Tree};
use sp_core::H256;
use std::{convert::TryInto, sync::Arc};
use substrate_parser::{compacts::find_compact, traits::AsMetadata, ShortSpecs};

use kampela_common::Encryption;

use crate::error::ErrorCompanion;

fn open_db(db_path: &str) -> Result<Db, ErrorCompanion> {
    open(db_path).map_err(ErrorCompanion::DbInternal)
}

fn open_tree(database: &Db, tree_name: &[u8]) -> Result<Tree, ErrorCompanion> {
    database
        .open_tree(tree_name)
        .map_err(ErrorCompanion::DbInternal)
}

/// Tree name for metadata storage
pub const METADATA: &[u8] = b"metadata";

/// Tree name for specs storage
pub const SPECS: &[u8] = b"specs";

#[derive(Debug, Decode, Encode)]
pub struct MetadataKey {
    pub genesis_hash: H256,
}

impl MetadataKey {
    pub fn as_db_key(&self) -> Vec<u8> {
        self.encode()
    }
}

#[derive(Debug)]
pub struct MetadataStorage {
    pub key: MetadataKey,
    pub value: RuntimeMetadataV14,
}

impl MetadataStorage {
    pub fn from_payload_prelude_cut(
        payload: &[u8],
        encryption: &Encryption,
    ) -> Result<Self, ErrorCompanion> {
        let mut position = encryption.key_length();
        let length_info = find_compact::<u32, _, _>(&payload, &mut (), position)
            .map_err(|_| ErrorCompanion::MetadataQrUnexpectedStructure)?;
        let meta_length = length_info.compact as usize;
        position = length_info.start_next_unit;
        match payload.get(position..position + meta_length) {
            Some(meta_slice) => {
                if !meta_slice.starts_with(META_RESERVED.to_le_bytes().as_ref()) {
                    return Err(ErrorCompanion::NoMetaPrefixQr);
                }
                let meta_decoded = RuntimeMetadata::decode(&mut &meta_slice[4..])
                    .map_err(|_| ErrorCompanion::MetadataQrDecode)?;
                let metadata = match meta_decoded {
                    RuntimeMetadata::V14(metadata) => metadata,
                    _ => return Err(ErrorCompanion::OnlyV14SupportedQr),
                };
                position += meta_length;
                match payload.get(position..position + H256::len_bytes()) {
                    Some(hash_slice) => {
                        let genesis_hash =
                            H256(hash_slice.try_into().expect("stable known length"));
                        Ok(Self {
                            key: MetadataKey { genesis_hash },
                            value: metadata,
                        })
                    }
                    None => Err(ErrorCompanion::TooShort),
                }
            }
            None => Err(ErrorCompanion::TooShort),
        }
    }

    pub fn write_in_db(&self, db_path: &str) -> Result<(), ErrorCompanion> {
        let database = open_db(db_path)?;
        let metadata_tree = open_tree(&database, METADATA)?;
        metadata_tree
            .insert(self.key.as_db_key(), self.value.encode())
            .map_err(ErrorCompanion::DbInternal)?;
        Ok(())
    }

    pub fn read_from_tree(
        metadata_tree: &Tree,
        genesis_hash: H256,
    ) -> Result<Self, ErrorCompanion> {
        let metadata_key = MetadataKey { genesis_hash };
        match metadata_tree.get(metadata_key.as_db_key()) {
            Ok(Some(encoded_meta_slice)) => {
                let value = RuntimeMetadataV14::decode(&mut &encoded_meta_slice[..])
                    .map_err(|_| ErrorCompanion::DecodeDbMetadataValue)?;
                Ok(Self {
                    key: metadata_key,
                    value,
                })
            }
            Ok(None) => Err(ErrorCompanion::NoMetadata(genesis_hash)),
            Err(e) => Err(ErrorCompanion::DbInternal(e)),
        }
    }

    pub fn read_from_db(db_path: &str, genesis_hash: H256) -> Result<Self, ErrorCompanion> {
        let database = open_db(db_path)?;
        let metadata_tree = open_tree(&database, METADATA)?;
        Self::read_from_tree(&metadata_tree, genesis_hash)
    }
}

#[derive(Clone, Debug, Decode, Encode, Eq, PartialEq)]
pub struct SpecsKey {
    pub encryption: Encryption,
    pub genesis_hash: H256,
}

impl SpecsKey {
    pub fn from_db_key(database_key: &IVec) -> Result<Self, ErrorCompanion> {
        Self::decode(&mut &database_key[..]).map_err(|_| ErrorCompanion::DecodeDbSpecsKey)
    }
    pub fn as_db_key(&self) -> Vec<u8> {
        self.encode()
    }
    pub fn show(&self) -> String {
        hex::encode(self.as_db_key())
    }
}

#[derive(Clone, Debug, Decode, Encode, Eq, PartialEq)]
pub struct SpecsUpstream {
    pub base58prefix: u16,
    pub color: String,
    pub decimals: u8,
    pub encryption: Encryption,
    pub genesis_hash: H256,
    pub logo: String,
    pub name: String,
    pub path_id: String,
    pub secondary_color: String,
    pub title: String,
    pub unit: String,
}

#[derive(Clone, Debug, Decode, Encode)]
pub struct SpecsValue {
    pub short_specs: ShortSpecs,
    pub title: String,
}

#[derive(Clone, Debug)]
pub struct SpecsStorage {
    pub key: SpecsKey,
    pub value: SpecsValue,
}

impl SpecsStorage {
    pub fn from_payload_prelude_cut(
        payload: &[u8],
        encryption: &Encryption,
    ) -> Result<Self, ErrorCompanion> {
        let mut position = encryption.key_length();
        let length_info = find_compact::<u32, _, _>(&payload, &mut (), position)
            .map_err(|_| ErrorCompanion::SpecsQrUnexpectedStructure)?;
        let encoded_specs_length = length_info.compact as usize;
        position = length_info.start_next_unit;
        match payload.get(position..position + encoded_specs_length) {
            Some(encoded_specs_slice) => {
                let specs_upstream = SpecsUpstream::decode(&mut &encoded_specs_slice[..])
                    .map_err(|_| ErrorCompanion::SpecsQrDecode)?;
                Ok(Self {
                    key: SpecsKey {
                        encryption: specs_upstream.encryption,
                        genesis_hash: specs_upstream.genesis_hash,
                    },
                    value: SpecsValue {
                        short_specs: ShortSpecs {
                            base58prefix: specs_upstream.base58prefix,
                            decimals: specs_upstream.decimals,
                            unit: specs_upstream.unit,
                        },
                        title: specs_upstream.title,
                    },
                })
            }
            None => Err(ErrorCompanion::TooShort),
        }
    }

    pub fn write_in_db(&self, db_path: &str) -> Result<(), ErrorCompanion> {
        let database = open_db(db_path)?;
        let specs_tree = open_tree(&database, SPECS)?;
        specs_tree
            .insert(self.key.as_db_key(), self.value.encode())
            .map_err(ErrorCompanion::DbInternal)?;
        Ok(())
    }

    pub fn read_from_db(
        db_path: &str,
        encryption: Encryption,
        genesis_hash: H256,
    ) -> Result<Self, ErrorCompanion> {
        let database = open_db(db_path)?;
        let specs_tree = open_tree(&database, SPECS)?;
        let specs_key = SpecsKey {
            encryption,
            genesis_hash,
        };
        match specs_tree.get(specs_key.as_db_key()) {
            Ok(Some(database_value)) => {
                let value = SpecsValue::decode(&mut &database_value[..])
                    .map_err(|_| ErrorCompanion::DecodeDbSpecsValue)?;
                Ok(Self {
                    key: specs_key,
                    value,
                })
            }
            Ok(None) => Err(ErrorCompanion::NoSpecs {
                encryption,
                genesis_hash,
            }),
            Err(e) => Err(ErrorCompanion::DbInternal(e)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SpecsDisplayElement {
    specs_key: SpecsKey,
    specs_value: SpecsValue,
    metadata_version: Option<String>,
}

impl SpecsDisplayElement {
    fn from_entry(
        (specs_key_slice, specs_value_slice): (IVec, IVec),
        metadata_tree: &Tree,
    ) -> Result<Self, ErrorCompanion> {
        let specs_key = SpecsKey::from_db_key(&specs_key_slice)?;
        let specs_value = SpecsValue::decode(&mut &specs_value_slice[..])
            .map_err(|_| ErrorCompanion::DecodeDbSpecsValue)?;
        let metadata_version = {
            if let Ok(metadata_storage) =
                MetadataStorage::read_from_tree(metadata_tree, specs_key.genesis_hash)
            {
                Some(
                    <RuntimeMetadataV14 as AsMetadata<()>>::spec_name_version(
                        &metadata_storage.value,
                    )
                    .map_err(ErrorCompanion::MetadataVersion)?
                    .printed_spec_version,
                )
            } else {
                None
            }
        };
        Ok(Self {
            specs_key,
            specs_value,
            metadata_version,
        })
    }
    fn title(&self) -> String {
        self.specs_value.title.to_owned()
    }
    fn version(&self) -> Option<String> {
        self.metadata_version.to_owned()
    }
}

#[derive(Debug)]
pub struct SpecsDisplay {
    display: Vec<SpecsDisplayElement>,
}

impl SpecsDisplay {
    pub fn new(db_path: &str) -> Result<Self, ErrorCompanion> {
        let database = open_db(db_path)?;
        let specs_tree = open_tree(&database, SPECS)?;
        let metadata_tree = open_tree(&database, METADATA)?;
        let mut display: Vec<SpecsDisplayElement> = Vec::new();
        for x in specs_tree.iter().flatten() {
            display.push(SpecsDisplayElement::from_entry(x, &metadata_tree)?)
        }
        Ok(Self { display })
    }

    pub fn get_all_keys(&self) -> Vec<Arc<SpecsKey>> {
        self.display
            .iter()
            .map(|a| Arc::new(a.specs_key.to_owned()))
            .collect()
    }

    pub fn title(&self, key: &SpecsKey) -> Result<Option<String>, ErrorCompanion> {
        let mut title = None;
        for element in self.display.iter() {
            if &element.specs_key == key {
                title = Some(element.title());
                break;
            }
        }
        Ok(title)
    }

    pub fn version(&self, key: &SpecsKey) -> Result<Option<String>, ErrorCompanion> {
        let mut version = None;
        for element in self.display.iter() {
            if &element.specs_key == key {
                version = element.version();
                break;
            }
        }
        Ok(version)
    }
}
