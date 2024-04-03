use std::sync::Arc;

use crate::fetch::{full_fetch, metadata_fetch, try_read_full_fetch, try_read_metadata_fetch};
use crate::selector::{Selector, POLKADOT_ADDRESS, WESTEND_ADDRESS};

#[test]
fn good_full_fetch() {
    assert!(try_read_full_fetch().unwrap().is_none());

    let fetch_result = full_fetch(POLKADOT_ADDRESS);
    assert!(fetch_result.is_ok());

    let mut read_result = try_read_full_fetch().unwrap();
    while read_result.is_none() {
        read_result = try_read_full_fetch().unwrap();
    }

    assert_eq!(read_result.unwrap().address, POLKADOT_ADDRESS);
    assert!(try_read_full_fetch().unwrap().is_none());
}

#[test]
fn good_metadata_fetch() {
    assert!(try_read_metadata_fetch().unwrap().is_none());

    let fetch_result = metadata_fetch(WESTEND_ADDRESS);
    assert!(fetch_result.is_ok());

    let mut read_result = try_read_metadata_fetch().unwrap();
    while read_result.is_none() {
        read_result = try_read_metadata_fetch().unwrap();
    }

    assert!(try_read_metadata_fetch().unwrap().is_none());
}

#[test]
#[ignore = "parallel fetches unavailable at the moment"]
fn selector_1() {
    let db_path = "../selector_1";
    let selector = Arc::new(Selector::new(db_path).unwrap());

    selector.add_new_element(WESTEND_ADDRESS, db_path).unwrap();
    let all_keys = selector.get_all_keys().unwrap();
    assert_eq!(all_keys.len(), 1);

    selector.update(&all_keys[0], db_path).unwrap();
    let all_keys = selector.get_all_keys().unwrap();
    assert_eq!(all_keys.len(), 1);

    selector.delete(&all_keys[0], db_path).unwrap();
    let all_keys = selector.get_all_keys().unwrap();
    assert!(all_keys.is_empty());

    std::fs::remove_dir_all(db_path).unwrap();
}
