use frame_metadata::v15::RuntimeMetadataV15;
use parity_scale_codec::{Decode, Encode};
use primitive_types::H256;
use sled::{open, Db, Transactional, Tree};
use substrate_parser::ShortSpecs;

use crate::error::ErrorCompanion;
use crate::fetch::try_read_full_fetch;

fn open_db(db_path: &str) -> Result<Db, ErrorCompanion> {
    open(db_path).map_err(ErrorCompanion::DbInternal)
}

fn open_tree(database: &Db, tree_name: &[u8]) -> Result<Tree, ErrorCompanion> {
    database
        .open_tree(tree_name)
        .map_err(ErrorCompanion::DbInternal)
}

/// Tree name for address book storage
pub const ADDRESS: &[u8] = b"address_book";

/// Tree name for metadata storage
pub const METADATA: &[u8] = b"metadata";

/// Tree name for specs storage
pub const SPECS: &[u8] = b"specs";

/// Key for the database entries: genesis hash.
///
/// If the genesis hash changes, all info must be entered again.
#[derive(Debug)]
pub struct Key {
    pub genesis_hash: H256,
}

impl Key {
    pub fn new(genesis_hash: H256) -> Self {
        Self { genesis_hash }
    }
    pub fn as_db_key(&self) -> Vec<u8> {
        self.genesis_hash.encode()
    }
}

#[derive(Debug)]
pub struct ValueAddress(pub String);

#[derive(Debug)]
pub struct ValueMetadata(pub RuntimeMetadataV15);

#[derive(Debug)]
pub struct ValueSpecs(pub ShortSpecs);

macro_rules! impl_try_get {
    ($($ty: ty, $inner_ty: ty, $tree: expr, $error: ident), *) => {
        $(
            impl $ty {
                pub fn try_get_tree(genesis_hash: H256, tree: &Tree) -> Result<Option<Self>, ErrorCompanion> {
                    let key = Key::new(genesis_hash);
                    match tree.get(key.as_db_key()) {
                        Ok(Some(encoded_slice)) => {
                            let value = <$inner_ty>::decode(&mut &encoded_slice[..])
                                .map_err(|_| ErrorCompanion::$error(genesis_hash))?;
                            Ok(Some(Self(value)))
                        }
                        Ok(None) => Ok(None),
                        Err(e) => Err(ErrorCompanion::DbInternal(e)),
                    }
                }
                pub fn try_get_db(genesis_hash: H256, db_path: &str) -> Result<Option<Self>, ErrorCompanion> {
                    let database = open_db(db_path)?;
                    let tree = open_tree(&database, $tree)?;
                    Self::try_get_tree(genesis_hash, &tree)
                }
                pub fn new(inner_value: &$inner_ty) -> Self {
                    Self(inner_value.to_owned())
                }
                pub fn into_value(&self) -> Vec<u8> {
                    self.0.encode()
                }
                pub fn inner(&self) -> &$inner_ty {
                    &self.0
                }
            }
        )*
    }
}

impl_try_get!(ValueAddress, String, ADDRESS, DecodeDbAddress);
impl_try_get!(
    ValueMetadata,
    RuntimeMetadataV15,
    METADATA,
    DecodeDbMetadata
);
impl_try_get!(ValueSpecs, ShortSpecs, SPECS, DecodeDbSpecs);

#[uniffi::export]
pub fn accepted_new_full_fetch(db_path: &str) -> Result<bool, ErrorCompanion> {
    if let Some(full_fetch) = try_read_full_fetch()? {
        let key = Key::new(full_fetch.genesis_hash).as_db_key();

        let database = open_db(db_path)?;
        let address_tree = open_tree(&database, ADDRESS)?;
        let metadata_tree = open_tree(&database, METADATA)?;
        let specs_tree = open_tree(&database, SPECS)?;

        (&address_tree, &metadata_tree, &specs_tree)
            .transaction(|(tx_address, tx_metadata, tx_specs)| {
                tx_address.insert(
                    key.as_slice(),
                    ValueAddress::new(&full_fetch.address).into_value(),
                )?;
                tx_address.flush();
                tx_metadata.insert(
                    key.as_slice(),
                    ValueMetadata::new(&full_fetch.metadata).into_value(),
                )?;
                tx_metadata.flush();
                tx_specs.insert(
                    key.as_slice(),
                    ValueSpecs::new(&full_fetch.specs).into_value(),
                )?;
                tx_specs.flush();
                Ok(())
            })
            .map_err(ErrorCompanion::DbTransaction)?;
        Ok(true)
    } else {
        Ok(false)
    }
}
