use crate::interface::{get_all_keys, is_updated, request_defaults};

#[test]
fn setup_defaults() {
    let db_path = "../defaults_test";

    request_defaults();

    let mut update_counter = 0;

    while update_counter < 3 {
        while !is_updated(db_path).unwrap() {}
        update_counter += 1;
    }

    let all_keys = get_all_keys(db_path).unwrap();
    assert_eq!(all_keys.len(), 3);

    std::fs::remove_dir_all(db_path).unwrap();
}
