use crate::fetch::{full_fetch, metadata_fetch, try_read_full_fetch, try_read_metadata_fetch};

const POLKADOT_ADDRESS: &str = "wss://rpc.polkadot.io";

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

    let fetch_result = metadata_fetch(POLKADOT_ADDRESS);
    assert!(fetch_result.is_ok());

    let mut read_result = try_read_metadata_fetch().unwrap();
    while read_result.is_none() {
        read_result = try_read_metadata_fetch().unwrap();
    }

    assert!(try_read_metadata_fetch().unwrap().is_none());
}
