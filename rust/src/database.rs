use frame_metadata::v15::RuntimeMetadataV15;
use parity_scale_codec::{Decode, Encode};
use primitive_types::H256;
use sled::{open, Db, IVec, Tree};
use substrate_parser::ShortSpecs;

use crate::error::ErrorCompanion;
use crate::UniffiCustomTypeConverter;

pub(crate) fn open_db(db_path: &str) -> Result<Db, ErrorCompanion> {
    open(db_path).map_err(ErrorCompanion::DbInternal)
}

pub(crate) fn open_tree(database: &Db, tree_name: &[u8]) -> Result<Tree, ErrorCompanion> {
    database
        .open_tree(tree_name)
        .map_err(ErrorCompanion::DbInternal)
}

/// Tree name for address book storage
pub const ADDRESS: &[u8] = b"address_book";

/// Tree name for metadata storage
pub const DATA: &[u8] = b"data";

/// Key for the database entries: genesis hash.
///
/// If the genesis hash changes, all info must be entered again.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ChainKey {
    pub genesis_hash: H256,
}

impl ChainKey {
    pub fn new(genesis_hash: H256) -> Self {
        Self { genesis_hash }
    }
    pub fn as_db_key(&self) -> Vec<u8> {
        self.genesis_hash.encode()
    }
    pub fn from_db_key(key_ivec: IVec) -> Result<Self, ErrorCompanion> {
        let genesis_hash =
            H256::decode(&mut &key_ivec[..]).map_err(|_| ErrorCompanion::DecodeDbKey(key_ivec))?;
        Ok(Self { genesis_hash })
    }
}

uniffi::custom_type!(ChainKey, String);

impl UniffiCustomTypeConverter for ChainKey {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        let inner = hex::decode(val).map_err(|_| uniffi::deps::anyhow::Error::msg(
            "Invalid hex encoding",
        ))?;
        Ok(ChainKey{ genesis_hash: H256( inner.try_into().map_err(|_| uniffi::deps::anyhow::Error::msg(
            "Incorrect length",
        ))?) })
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        hex::encode(obj.genesis_hash.0)
    }
}

/*
unsafe impl<UT> uniffi::Lower<UT> for ChainKey {
    type FfiType = ChainKey;
    const TYPE_ID_META: uniffi::MetadataBuffer = uniffi::MetadataBuffer {
        bytes: [0u8; 16384],
        size: 64usize,
    };
    fn lower(obj: Self) -> Self::FfiType {
        obj
    }
    fn write(obj: Self, buf: &mut Vec<u8>) {
        buf.copy_from_slice(obj.genesis_hash.0.as_slice())
    }
}

unsafe impl<UT> uniffi::Lift<UT> for ChainKey {
    type FfiType = ChainKey;
    const TYPE_ID_META: uniffi::MetadataBuffer = uniffi::MetadataBuffer {
        bytes: [0u8; 16384],
        size: 64usize,
    };
    fn try_lift(v: Self::FfiType) -> uniffi::Result<Self> {
        Ok(v)
    }
    fn try_read(buf: &mut &[u8]) -> uniffi::Result<Self> {
        let slice = buf.get(..32).ok_or(uniffi::deps::anyhow::Error::msg(
            "Not enough bytes in buffer",
        ))?;
        Ok(Self {
            genesis_hash: H256(slice.try_into().expect("static length, always fits")),
        })
    }
}

impl uniffi::FfiDefault for ChainKey {
    fn ffi_default() -> Self {
        ChainKey {
            genesis_hash: H256([0; 32]),
        }
    }
}
*/
#[derive(Debug)]
pub struct ValueAddress(pub String);

#[derive(Debug)]
pub struct ValueMetadataSpecs(pub MetadataSpecs);

#[derive(Clone, Debug, Decode, Encode)]
pub struct MetadataSpecs {
    pub metadata: RuntimeMetadataV15,
    pub specs: ShortSpecs,
}

macro_rules! impl_try_get {
    ($($ty: ty, $inner_ty: ty, $tree: expr, $error: ident), *) => {
        $(
            impl $ty {
                pub fn from_db_value(value_ivec: IVec, key: &ChainKey) -> Result<Self, ErrorCompanion> {
                    let value = <$inner_ty>::decode(&mut &value_ivec[..]).map_err(|_| ErrorCompanion::$error(key.genesis_hash))?;
                    Ok(Self(value))
                }
                pub fn try_get_tree(genesis_hash: H256, tree: &Tree) -> Result<Option<Self>, ErrorCompanion> {
                    let key = ChainKey::new(genesis_hash);
                    match tree.get(key.as_db_key()) {
                        Ok(Some(value_ivec)) => Ok(Some(Self::from_db_value(value_ivec, &key)?)),
                        Ok(None) => Ok(None),
                        Err(e) => Err(ErrorCompanion::DbInternal(e)),
                    }
                }
                pub fn try_get_db(genesis_hash: H256, db_path: &str) -> Result<Option<Self>, ErrorCompanion> {
                    let database = open_db(db_path)?;
                    let tree = open_tree(&database, $tree)?;
                    Self::try_get_tree(genesis_hash, &tree)
                }
                pub fn new(inner_value: $inner_ty) -> Self {
                    Self(inner_value)
                }
                pub fn into_value(&self) -> Vec<u8> {
                    self.0.encode()
                }
                pub fn inner(self) -> $inner_ty {
                    self.0
                }
            }
        )*
    }
}

impl_try_get!(ValueAddress, String, ADDRESS, DecodeDbAddress);
impl_try_get!(
    ValueMetadataSpecs,
    MetadataSpecs,
    DATA,
    DecodeDbMetadataSpecs
);
