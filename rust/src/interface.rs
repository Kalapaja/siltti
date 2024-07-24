//! Main interface

use frame_metadata::v15::RuntimeMetadataV15;
use primitive_types::H256;
use sled::Transactional;
use substrate_parser::traits::SpecNameVersion;

use crate::database::{
    open_db, open_tree, ChainKey, MetadataSpecs, ValueAddress, ValueMetadataSpecs, ADDRESS, DATA,
};
use crate::error::ErrorCompanion;
use crate::fetch::{full_fetch, metadata_fetch, try_read, FetchData, Fetched};

#[derive(Clone, Debug, uniffi::Object)]
pub struct SelectorElement {
    pub address: String,
    pub key: ChainKey,
    pub spec_name_metadata_version: SpecNameVersion,
}

#[uniffi::export]
pub fn is_updated(db_path: &str) -> Result<bool, ErrorCompanion> {
    match try_read()? {
        Some(Fetched::Whole { fetch_data }) => {
            accept_full_fetch(fetch_data, db_path)?;
            Ok(true)
        }
        Some(Fetched::Partial {
            genesis_hash,
            metadata,
        }) => {
            accept_metadata_fetch(genesis_hash, metadata, db_path)?;
            Ok(true)
        }
        None => Ok(false),
    }
}

#[uniffi::export]
pub fn request_defaults() {
    for address in ADDRESS_BOOK {
        full_fetch(address)
    }
}

#[uniffi::export]
pub fn get_all_keys(db_path: &str) -> Result<Vec<ChainKey>, ErrorCompanion> {
    let database = open_db(db_path)?;
    let data_tree = open_tree(&database, DATA)?;
    let mut key_set: Vec<ChainKey> = Vec::new();
    for (key_ivec, _) in data_tree.iter().flatten() {
        key_set.push(ChainKey::from_db_key(key_ivec)?);
    }
    key_set.sort();
    Ok(key_set)
}

#[uniffi::export]
pub fn delete_by_key(chain_key: ChainKey, db_path: &str) -> Result<(), ErrorCompanion> {
    let database = open_db(db_path)?;
    let address_tree = open_tree(&database, ADDRESS)?;
    let data_tree = open_tree(&database, DATA)?;

    let key_to_delete = chain_key.as_db_key();

    (&address_tree, &data_tree)
        .transaction(|(tx_address, tx_data)| {
            tx_address.remove(key_to_delete.as_slice())?;
            tx_address.flush();
            tx_data.remove(key_to_delete.as_slice())?;
            tx_data.flush();
            Ok(())
        })
        .map_err(ErrorCompanion::DbTransaction)?;
    Ok(())
}

#[uniffi::export]
pub fn request_update_by_key(chain_key: ChainKey, db_path: &str) -> Result<(), ErrorCompanion> {
    let address = match ValueAddress::try_get_db(chain_key.genesis_hash, db_path)? {
        Some(a) => a.inner(),
        None => return Err(ErrorCompanion::LostAddress(chain_key.genesis_hash)),
    };
    metadata_fetch(chain_key.genesis_hash, &address);
    Ok(())
}

#[uniffi::export]
pub fn request_full_fetch(address: &str) {
    full_fetch(address)
}

fn accept_full_fetch(fetch_data: FetchData, db_path: &str) -> Result<(), ErrorCompanion> {
    let database = open_db(db_path)?;
    let address_tree = open_tree(&database, ADDRESS)?;
    let data_tree = open_tree(&database, DATA)?;

    let key_to_insert = ChainKey::new(fetch_data.genesis_hash).as_db_key();
    let value_address_to_insert = ValueAddress::new(fetch_data.address).into_value();
    let value_metadata_specs_to_insert = ValueMetadataSpecs::new(MetadataSpecs {
        metadata: fetch_data.metadata,
        specs: fetch_data.specs,
    })
    .into_value();

    (&address_tree, &data_tree)
        .transaction(|(tx_address, tx_data)| {
            tx_address.insert(key_to_insert.as_slice(), value_address_to_insert.as_slice())?;
            tx_address.flush();
            tx_data.insert(
                key_to_insert.as_slice(),
                value_metadata_specs_to_insert.as_slice(),
            )?;
            tx_data.flush();
            Ok(())
        })
        .map_err(ErrorCompanion::DbTransaction)?;
    Ok(())
}

fn accept_metadata_fetch(
    genesis_hash: H256,
    metadata: RuntimeMetadataV15,
    db_path: &str,
) -> Result<(), ErrorCompanion> {
    let database = open_db(db_path)?;
    let data_tree = open_tree(&database, DATA)?;
    let mut value_metadata_specs = match ValueMetadataSpecs::try_get_tree(genesis_hash, &data_tree)?
    {
        Some(a) => a,
        None => {
            return Err(ErrorCompanion::MetadataFetchWithoutExistingEntry(
                genesis_hash,
            ))
        }
    };
    value_metadata_specs.0.metadata = metadata;

    (data_tree)
        .transaction(|tx_data| {
            tx_data.insert(
                ChainKey::new(genesis_hash).as_db_key().as_slice(),
                value_metadata_specs.into_value(),
            )?;
            tx_data.flush();
            Ok(())
        })
        .map_err(ErrorCompanion::DbTransaction)?;
    Ok(())
}

pub const KUSAMA_ADDRESS: &str = "ws://kusama.api.onfinality.io/public-ws";
pub const POLKADOT_ADDRESS: &str = "ws://polkadot.api.onfinality.io/public-ws";
pub const WESTEND_ADDRESS: &str = "ws://westend.api.onfinality.io/public-ws";

pub const ADDRESS_BOOK: &[&str] = &[KUSAMA_ADDRESS, POLKADOT_ADDRESS, WESTEND_ADDRESS];
