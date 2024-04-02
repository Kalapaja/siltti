//! Main interface

use frame_metadata::v15::RuntimeMetadataV15;
use sled::{IVec, Transactional};
use std::sync::{Arc, RwLock};
use substrate_parser::traits::{AsMetadata, SpecNameVersion};

use crate::database::{
    open_db, open_tree, Key, MetadataSpecs, ValueAddress, ValueMetadataSpecs, ADDRESS, DATA,
};
use crate::error::ErrorCompanion;
use crate::fetch::{full_fetch, metadata_fetch, try_read_full_fetch, try_read_metadata_fetch};

#[derive(Clone, Debug, uniffi::Object)]
pub struct SelectorElement {
    is_selected: bool,
    key: Key,
    spec_name_metadata_version: SpecNameVersion,
}

impl SelectorElement {
    fn add_new(address: &str, db_path: &str) -> Result<Self, ErrorCompanion> {
        full_fetch(address)?;
        let mut accepted_metadata = accepted_full_fetch(db_path)?;
        loop {
            match accepted_metadata {
                Some(new_selector_element) => return Ok(new_selector_element),
                None => {
                    accepted_metadata = accepted_full_fetch(db_path)?;
                }
            }
        }
    }

    fn update(&mut self, db_path: &str) -> Result<(), ErrorCompanion> {
        let address = match ValueAddress::try_get_db(self.key.genesis_hash, db_path)? {
            Some(a) => a.inner(),
            None => return Err(ErrorCompanion::LostAddress(self.key.genesis_hash)),
        };
        metadata_fetch(&address)?;
        let mut accepted_metadata = accepted_metadata_fetch(self.key, db_path)?;
        loop {
            match accepted_metadata {
                Some(accepted_spec_name_version) => {
                    self.spec_name_metadata_version = accepted_spec_name_version;
                    return Ok(());
                }
                None => {
                    accepted_metadata = accepted_metadata_fetch(self.key, db_path)?;
                }
            }
        }
    }

    fn delete_from_database(&self, db_path: &str) -> Result<(), ErrorCompanion> {
        let database = open_db(db_path)?;
        let address_tree = open_tree(&database, ADDRESS)?;
        let data_tree = open_tree(&database, DATA)?;

        let key_to_delete = self.key.as_db_key();

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

    fn from_entry((key_ivec, value_ivec): (IVec, IVec)) -> Result<Self, ErrorCompanion> {
        let key = Key::from_db_key(key_ivec)?;
        let metadata_specs = ValueMetadataSpecs::from_db_value(value_ivec, &key)?.inner();

        let spec_name_metadata_version =
            <RuntimeMetadataV15 as AsMetadata<()>>::spec_name_version(&metadata_specs.metadata)
                .map_err(ErrorCompanion::MetadataVersion)?;
        Ok(Self {
            is_selected: false,
            key,
            spec_name_metadata_version,
        })
    }
    fn toggle(&mut self) {
        self.is_selected = !self.is_selected;
    }
    fn make_selected(&mut self) {
        self.is_selected = true;
    }
    fn make_deselected(&mut self) {
        self.is_selected = false;
    }
    fn name(&self) -> String {
        self.spec_name_metadata_version.spec_name.to_owned()
    }
    fn version(&self) -> String {
        self.spec_name_metadata_version
            .printed_spec_version
            .to_owned()
    }
    fn is_selected(&self) -> bool {
        self.is_selected
    }
    fn key(&self) -> Key {
        self.key.to_owned()
    }
}

#[derive(Debug, uniffi::Object)]
pub struct Selector {
    selector: RwLock<Vec<SelectorElement>>,
}

#[uniffi::export]
impl Selector {
    #[uniffi::constructor]
    pub fn new(db_path: &str) -> Result<Self, ErrorCompanion> {
        let database = open_db(db_path)?;
        let data_tree = open_tree(&database, DATA)?;
        let mut selector: Vec<SelectorElement> = Vec::new();
        for x in data_tree.iter().flatten() {
            selector.push(SelectorElement::from_entry(x)?)
        }
        Ok(Self {
            selector: RwLock::new(selector),
        })
    }

    #[uniffi::method]
    pub fn add_new_element(
        self: &Arc<Self>,
        address: &str,
        db_path: &str,
    ) -> Result<(), ErrorCompanion> {
        let new_element = SelectorElement::add_new(address, db_path)?;
        let mut selector = self
            .selector
            .write()
            .map_err(|_| ErrorCompanion::PoisonedLockSelector)?;
        selector.push(new_element);
        Ok(())
    }

    #[uniffi::method]
    pub fn update(self: &Arc<Self>, key: &Key, db_path: &str) -> Result<(), ErrorCompanion> {
        let mut selector = self
            .selector
            .write()
            .map_err(|_| ErrorCompanion::PoisonedLockSelector)?;
        let mut found_index = None;
        for (index, element) in selector.iter().enumerate() {
            if element.key == *key {
                found_index = Some(index);
                break;
            }
        }
        if let Some(index) = found_index {
            let mut element_to_update = selector.swap_remove(index);
            element_to_update.update(db_path)?;
            selector.push(element_to_update)
        }
        Ok(())
    }

    #[uniffi::method]
    pub fn delete(self: &Arc<Self>, key: &Key, db_path: &str) -> Result<(), ErrorCompanion> {
        let mut selector = self
            .selector
            .write()
            .map_err(|_| ErrorCompanion::PoisonedLockSelector)?;
        let mut found_index = None;
        for (index, element) in selector.iter().enumerate() {
            if element.key == *key {
                found_index = Some(index);
                break;
            }
        }
        if let Some(index) = found_index {
            let removed_element = selector.swap_remove(index);
            removed_element.delete_from_database(db_path)?;
        }
        Ok(())
    }

    #[uniffi::method]
    pub fn get_all_keys(&self) -> Result<Vec<Arc<Key>>, ErrorCompanion> {
        let selector = self
            .selector
            .read()
            .map_err(|_| ErrorCompanion::PoisonedLockSelector)?;
        Ok(selector.iter().map(|a| Arc::new(a.key())).collect())
    }

    /// Name, optional. Option must stay, despite being always Some(_) if the
    /// interface is not damaged, or the front would freeze with error.
    #[uniffi::method]
    pub fn name(&self, key: &Key) -> Result<Option<String>, ErrorCompanion> {
        let selector = self
            .selector
            .read()
            .map_err(|_| ErrorCompanion::PoisonedLockSelector)?;
        for element in selector.iter() {
            if &element.key() == key {
                return Ok(Some(element.name()));
            }
        }
        Ok(None)
    }

    #[uniffi::method]
    pub fn version(&self, key: &Key) -> Result<Option<String>, ErrorCompanion> {
        let selector = self
            .selector
            .read()
            .map_err(|_| ErrorCompanion::PoisonedLockSelector)?;
        for element in selector.iter() {
            if &element.key() == key {
                return Ok(Some(element.version()));
            }
        }
        Ok(None)
    }

    #[uniffi::method]
    pub fn is_selected(&self, key: &Key) -> Result<Option<bool>, ErrorCompanion> {
        let selector = self
            .selector
            .read()
            .map_err(|_| ErrorCompanion::PoisonedLockSelector)?;
        for element in selector.iter() {
            if &element.key() == key {
                return Ok(Some(element.is_selected()));
            }
        }
        Ok(None)
    }

    #[uniffi::method]
    pub fn toggle(self: &Arc<Self>, key: &Key) -> Result<(), ErrorCompanion> {
        let mut selector = self
            .selector
            .write()
            .map_err(|_| ErrorCompanion::PoisonedLockSelector)?;
        for element in selector.iter_mut() {
            if &element.key() == key {
                element.toggle();
                break;
            }
        }
        Ok(())
    }

    #[uniffi::method]
    pub fn select_all(self: &Arc<Self>) -> Result<(), ErrorCompanion> {
        let mut selector = self
            .selector
            .write()
            .map_err(|_| ErrorCompanion::PoisonedLockSelector)?;
        for element in selector.iter_mut() {
            element.make_selected()
        }
        Ok(())
    }

    #[uniffi::method]
    pub fn deselect_all(self: &Arc<Self>) -> Result<(), ErrorCompanion> {
        let mut selector = self
            .selector
            .write()
            .map_err(|_| ErrorCompanion::PoisonedLockSelector)?;
        for element in selector.iter_mut() {
            element.make_deselected()
        }
        Ok(())
    }
}

pub fn accepted_full_fetch(db_path: &str) -> Result<Option<SelectorElement>, ErrorCompanion> {
    if let Some(full_fetch) = try_read_full_fetch()? {
        let key = Key::new(full_fetch.genesis_hash);

        let out = SelectorElement {
            is_selected: false,
            key,
            spec_name_metadata_version: <RuntimeMetadataV15 as AsMetadata<()>>::spec_name_version(
                &full_fetch.metadata,
            )
            .map_err(ErrorCompanion::MetadataVersion)?,
        };

        let database = open_db(db_path)?;
        let address_tree = open_tree(&database, ADDRESS)?;
        let data_tree = open_tree(&database, DATA)?;

        let key_to_insert = key.as_db_key();
        let value_address_to_insert = ValueAddress::new(full_fetch.address.to_owned()).into_value();
        let value_metadata_specs_to_insert = ValueMetadataSpecs::new(MetadataSpecs {
            metadata: full_fetch.metadata.to_owned(),
            specs: full_fetch.specs.to_owned(),
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
        Ok(Some(out))
    } else {
        Ok(None)
    }
}

pub fn accepted_metadata_fetch(
    key: Key,
    db_path: &str,
) -> Result<Option<SpecNameVersion>, ErrorCompanion> {
    if let Some(metadata_fetch) = try_read_metadata_fetch()? {
        let out = <RuntimeMetadataV15 as AsMetadata<()>>::spec_name_version(&metadata_fetch)
            .map_err(ErrorCompanion::MetadataVersion)?;

        let database = open_db(db_path)?;
        let data_tree = open_tree(&database, DATA)?;
        let mut value_metadata_specs =
            match ValueMetadataSpecs::try_get_tree(key.genesis_hash, &data_tree)? {
                Some(a) => a,
                None => {
                    return Err(ErrorCompanion::MetadataFetchWithoutExistingEntry(
                        key.genesis_hash,
                    ))
                }
            };
        value_metadata_specs.0.metadata = metadata_fetch;

        (data_tree)
            .transaction(|tx_data| {
                tx_data.insert(
                    key.as_db_key().as_slice(),
                    value_metadata_specs.into_value(),
                )?;
                tx_data.flush();
                Ok(())
            })
            .map_err(ErrorCompanion::DbTransaction)?;
        Ok(Some(out))
    } else {
        Ok(None)
    }
}
