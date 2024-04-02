use lazy_static::lazy_static;
use regex::Regex;

use crate::error::{ErrorCompanion, NotHex};

lazy_static! {
    /// Regex to add port to addresses that have no port specified.
    static ref PORT: Regex = Regex::new(r"^(?P<body>wss://[^/]*?)(?P<port>:[0-9]+)?(?P<tail>/.*)?$").expect("known value");
}

/// Supply address with port if needed.
///
/// Transform address as it is displayed to user in <https://polkadot.js.org/>
/// to address with port added if necessary that could be fed to `jsonrpsee`
/// client.
///
/// The port is set here to default 443 if there is no port specified in
/// address itself, since default port in `jsonrpsee` is unavailable for now.
///
/// See for details <https://github.com/paritytech/jsonrpsee/issues/554>
///
/// Some addresses have port specified, and should be left as is.
pub fn address_with_port(rpc_address: &str) -> String {
    match PORT.captures(rpc_address) {
        Some(caps) => {
            if caps.name("port").is_some() {
                rpc_address.to_string()
            } else if let Some(tail) = caps.name("tail") {
                format!("{}:443{}", &caps["body"], tail.as_str())
            } else {
                format!("{}:443", &caps["body"])
            }
        }
        None => rpc_address.to_string(),
    }
}

pub fn unhex(hex_data: &str, what_is_hex: NotHex) -> Result<Vec<u8>, ErrorCompanion> {
    if let Some(stripped) = hex_data.strip_prefix("0x") {
        hex::decode(stripped).map_err(|_| ErrorCompanion::NotHex(what_is_hex))
    } else {
        hex::decode(hex_data).map_err(|_| ErrorCompanion::NotHex(what_is_hex))
    }
}
