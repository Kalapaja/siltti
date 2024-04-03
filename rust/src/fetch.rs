use frame_metadata::{v15::RuntimeMetadataV15, RuntimeMetadata};
use jsonrpsee::core::client::ClientT;
use jsonrpsee::rpc_params;
use jsonrpsee::ws_client::WsClientBuilder;
use lazy_static::lazy_static;
use parity_scale_codec::DecodeAll;
use primitive_types::H256;
use serde_json::{Map, Number, Value};
use std::sync::Mutex;
use substrate_parser::{cards::ParsedData, decode_all_as_type, ShortSpecs};
use tokio::{
    runtime::Runtime,
    sync::oneshot::{channel, error::TryRecvError, Receiver},
};

use crate::error::{ErrorCompanion, NotHex};
use crate::utils::{address_with_port, unhex};

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().expect("Runtime initiation failed.");
}

lazy_static! {
    static ref RECEIVER_FULL_FETCH: Mutex<Option<Receiver<FetchData>>> = Mutex::new(None);
}

lazy_static! {
    static ref RECEIVER_METADATA_FETCH: Mutex<Option<Receiver<RuntimeMetadataV15>>> =
        Mutex::new(None);
}

pub fn try_read_full_fetch() -> Result<Option<FetchData>, ErrorCompanion> {
    let guard = RECEIVER_FULL_FETCH.lock();
    match guard {
        Ok(mut mg) => match *mg {
            Some(ref mut a) => match a.try_recv() {
                Ok(b) => {
                    let out = Some(b);
                    *mg = None;
                    Ok(out)
                }
                Err(TryRecvError::Empty) => Ok(None),
                Err(TryRecvError::Closed) => Err(ErrorCompanion::ReceiverClosed),
            },
            None => Ok(None),
        },
        Err(_) => Err(ErrorCompanion::ReceiverGuardPoisoned),
    }
}

pub fn try_read_metadata_fetch() -> Result<Option<RuntimeMetadataV15>, ErrorCompanion> {
    let guard = RECEIVER_METADATA_FETCH.lock();
    match guard {
        Ok(mut mg) => match *mg {
            Some(ref mut a) => match a.try_recv() {
                Ok(b) => {
                    let out = Some(b);
                    *mg = None;
                    Ok(out)
                }
                Err(TryRecvError::Empty) => Ok(None),
                Err(TryRecvError::Closed) => Err(ErrorCompanion::ReceiverClosed),
            },
            None => Ok(None),
        },
        Err(_) => Err(ErrorCompanion::ReceiverGuardPoisoned),
    }
}

#[derive(Debug)]
pub struct FetchData {
    pub address: String,
    pub genesis_hash: H256,
    pub metadata: RuntimeMetadataV15,
    pub specs: ShortSpecs,
}

pub fn full_fetch(address: &str) -> Result<(), ErrorCompanion> {
    let (tx, rx) = channel();
    let address = address.to_string();
    let address_with_port = address_with_port(&address);
    RUNTIME.spawn(async move {
        let client = WsClientBuilder::default()
            .build(&address_with_port)
            .await
            .map_err(ErrorCompanion::Client)?;

        // fetch current block hash, to request later the metadata and specs for
        // the same block
        let block_hash_request: Value = client
            .request("chain_getBlockHash", rpc_params![])
            .await
            .map_err(ErrorCompanion::Client)?;
        let block_hash_string = match block_hash_request {
            Value::String(x) => x,
            _ => return Err(ErrorCompanion::BlockHashFormat),
        };

        // fetch genesis hash, must be a hexadecimal string transformable into
        // H256 format
        let genesis_hash_request: Value = client
            .request(
                "chain_getBlockHash",
                rpc_params![Value::Number(Number::from(0u8))],
            )
            .await
            .map_err(ErrorCompanion::Client)?;
        let genesis_hash = match genesis_hash_request {
            Value::String(x) => {
                let genesis_hash_raw = unhex(&x, NotHex::GenesisHash)?;
                H256(
                    genesis_hash_raw
                        .try_into()
                        .map_err(|_| ErrorCompanion::GenesisHashLength)?,
                )
            }
            _ => return Err(ErrorCompanion::GenesisHashFormat),
        };

        // fetch metadata at known block
        let metadata_request: Value = client
            .request(
                "state_call",
                rpc_params![
                    "Metadata_metadata_at_version",
                    "0f000000",
                    &block_hash_string
                ],
            )
            .await
            .map_err(ErrorCompanion::Client)?;
        let metadata = match metadata_request {
            Value::String(x) => {
                let metadata_request_raw = unhex(&x, NotHex::Metadata)?;
                let maybe_metadata_raw =
                    Option::<Vec<u8>>::decode_all(&mut &metadata_request_raw[..])
                        .map_err(|_| ErrorCompanion::RawMetadataNotDecodeable)?;
                if let Some(meta_v15_bytes) = maybe_metadata_raw {
                    if meta_v15_bytes.starts_with(b"meta") {
                        match RuntimeMetadata::decode_all(&mut &meta_v15_bytes[4..]) {
                            Ok(RuntimeMetadata::V15(runtime_metadata_v15)) => runtime_metadata_v15,
                            Ok(_) => return Err(ErrorCompanion::NoMetadataV15),
                            Err(_) => return Err(ErrorCompanion::MetadataNotDecodeable),
                        }
                    } else {
                        return Err(ErrorCompanion::NoMetaPrefix);
                    }
                } else {
                    return Err(ErrorCompanion::NoMetadataV15);
                }
            }
            _ => return Err(ErrorCompanion::MetadataFormat),
        };

        // fetch specs at known block
        let specs_request: Value = client
            .request("system_properties", rpc_params![&block_hash_string])
            .await
            .map_err(ErrorCompanion::Client)?;
        let specs = match specs_request {
            Value::Object(properties) => system_properties_to_short_specs(&properties, &metadata)?,
            _ => return Err(ErrorCompanion::PropertiesFormat),
        };

        let fetch_data = FetchData {
            address,
            genesis_hash,
            metadata,
            specs,
        };

        tx.send(fetch_data).map_err(|_| ErrorCompanion::NotSent)
    });
    let guard = RECEIVER_FULL_FETCH.lock();
    match guard {
        Ok(mut a) => {
            if a.is_none() {*a = Some(rx);}
            else {panic!("Parallel fetches unavailable at the moment.")}
        }
        Err(_) => return Err(ErrorCompanion::ReceiverGuardPoisoned),
    }
    Ok(())
}

pub fn metadata_fetch(address: &str) -> Result<(), ErrorCompanion> {
    let (tx, rx) = channel();
    let address_with_port = address_with_port(address);
    RUNTIME.spawn(async move {
        let client = WsClientBuilder::default()
            .build(&address_with_port)
            .await
            .map_err(ErrorCompanion::Client)?;

        // fetch metadata at latest block
        let metadata_request: Value = client
            .request(
                "state_call",
                rpc_params!["Metadata_metadata_at_version", "0f000000"],
            )
            .await
            .map_err(ErrorCompanion::Client)?;
        let metadata = match metadata_request {
            Value::String(x) => {
                let metadata_request_raw = unhex(&x, NotHex::Metadata)?;
                let maybe_metadata_raw =
                    Option::<Vec<u8>>::decode_all(&mut &metadata_request_raw[..])
                        .map_err(|_| ErrorCompanion::RawMetadataNotDecodeable)?;
                if let Some(meta_v15_bytes) = maybe_metadata_raw {
                    if meta_v15_bytes.starts_with(b"meta") {
                        match RuntimeMetadata::decode_all(&mut &meta_v15_bytes[4..]) {
                            Ok(RuntimeMetadata::V15(runtime_metadata_v15)) => runtime_metadata_v15,
                            Ok(_) => return Err(ErrorCompanion::NoMetadataV15),
                            Err(_) => return Err(ErrorCompanion::MetadataNotDecodeable),
                        }
                    } else {
                        return Err(ErrorCompanion::NoMetaPrefix);
                    }
                } else {
                    return Err(ErrorCompanion::NoMetadataV15);
                }
            }
            _ => return Err(ErrorCompanion::MetadataFormat),
        };

        tx.send(metadata).map_err(|_| ErrorCompanion::NotSent)
    });
    let guard = RECEIVER_METADATA_FETCH.lock();
    match guard {
        Ok(mut a) => {
            if a.is_none() {*a = Some(rx);}
            else {panic!("Parallel fetches unavailable at the moment.")}
        }
        Err(_) => return Err(ErrorCompanion::ReceiverGuardPoisoned),
    }
    Ok(())
}

fn optional_prefix_from_meta(metadata: &RuntimeMetadataV15) -> Option<u16> {
    let mut base58_prefix_data = None;
    for pallet in &metadata.pallets {
        if pallet.name == "System" {
            for system_constant in &pallet.constants {
                if system_constant.name == "SS58Prefix" {
                    base58_prefix_data = Some((&system_constant.value, &system_constant.ty));
                    break;
                }
            }
            break;
        }
    }
    if let Some((value, ty_symbol)) = base58_prefix_data {
        match decode_all_as_type::<&[u8], (), RuntimeMetadataV15>(
            ty_symbol,
            &value.as_ref(),
            &mut (),
            &metadata.types,
        ) {
            Ok(extended_data) => match extended_data.data {
                ParsedData::PrimitiveU8 {
                    value,
                    specialty: _,
                } => Some(value.into()),
                ParsedData::PrimitiveU16 {
                    value,
                    specialty: _,
                } => Some(value),
                ParsedData::PrimitiveU32 {
                    value,
                    specialty: _,
                } => value.try_into().ok(),
                ParsedData::PrimitiveU64 {
                    value,
                    specialty: _,
                } => value.try_into().ok(),
                ParsedData::PrimitiveU128 {
                    value,
                    specialty: _,
                } => value.try_into().ok(),
                _ => None,
            },
            Err(_) => None,
        }
    } else {
        None
    }
}

fn base58prefix(
    x: &Map<String, Value>,
    optional_prefix_from_meta: Option<u16>,
) -> Result<u16, ErrorCompanion> {
    let base58prefix: u16 = match x.get("ss58Format") {
        // base58 prefix is fetched in `system_properties` rpc call
        Some(a) => match a {
            // base58 prefix value is a number
            Value::Number(b) => match b.as_u64() {
                // number is integer and could be represented as `u64` (the only
                // suitable interpretation available for `Number`)
                Some(c) => match c.try_into() {
                    // this `u64` fits into `u16` that base58 prefix is supposed
                    // to be
                    Ok(d) => match optional_prefix_from_meta {
                        // base58 prefix was found in `SS58Prefix` constant of
                        // the network metadata
                        //
                        // check that the prefixes match
                        Some(prefix_from_meta) => {
                            if prefix_from_meta == d {
                                d
                            } else {
                                return Err(ErrorCompanion::Base58PrefixMismatch {
                                    specs: d,
                                    meta: prefix_from_meta,
                                });
                            }
                        }

                        // no base58 prefix was found in the network metadata
                        None => d,
                    },

                    // `u64` value does not fit into `u16` base58 prefix format,
                    // this is an error
                    Err(_) => {
                        return Err(ErrorCompanion::Base58PrefixFormatNotSupported {
                            value: a.to_string(),
                        })
                    }
                },

                // base58 prefix value could not be presented as `u64` number,
                // this is an error
                None => {
                    return Err(ErrorCompanion::Base58PrefixFormatNotSupported {
                        value: a.to_string(),
                    })
                }
            },

            // base58 prefix value is not a number, this is an error
            _ => {
                return Err(ErrorCompanion::Base58PrefixFormatNotSupported {
                    value: a.to_string(),
                })
            }
        },

        // no base58 prefix fetched in `system_properties` rpc call
        None => match optional_prefix_from_meta {
            // base58 prefix was found in `SS58Prefix` constant of the network
            // metadata
            Some(prefix_from_meta) => prefix_from_meta,

            // no base58 prefix at all, this is an error
            None => return Err(ErrorCompanion::NoBase58Prefix),
        },
    };
    Ok(base58prefix)
}

fn decimals(x: &Map<String, Value>) -> Result<u8, ErrorCompanion> {
    match x.get("tokenDecimals") {
        // decimals info is fetched in `system_properties` rpc call
        Some(a) => match a {
            // fetched decimals value is a number
            Value::Number(b) => match b.as_u64() {
                // number is integer and could be represented as `u64` (the only
                // suitable interpretation available for `Number`)
                Some(c) => match c.try_into() {
                    // this `u64` fits into `u8` that decimals is supposed to be
                    Ok(d) => Ok(d),

                    // this `u64` does not fit into `u8`, this is an error
                    Err(_) => Err(ErrorCompanion::DecimalsFormatNotSupported {
                        value: a.to_string(),
                    }),
                },

                // number could not be represented as `u64`, this is an error
                None => Err(ErrorCompanion::DecimalsFormatNotSupported {
                    value: a.to_string(),
                }),
            },

            // fetched decimals is an array
            Value::Array(b) => {
                // array with only one element
                if b.len() == 1 {
                    // this element is a number, process same as
                    // `Value::Number(_)`
                    if let Value::Number(c) = &b[0] {
                        match c.as_u64() {
                            // number is integer and could be represented as
                            // `u64` (the only suitable interpretation available
                            // for `Number`)
                            Some(d) => match d.try_into() {
                                // this `u64` fits into `u8` that decimals is
                                // supposed to be
                                Ok(f) => Ok(f),

                                // this `u64` does not fit into `u8`, this is an
                                // error
                                Err(_) => Err(ErrorCompanion::DecimalsFormatNotSupported {
                                    value: a.to_string(),
                                }),
                            },

                            // number could not be represented as `u64`, this is
                            // an error
                            None => Err(ErrorCompanion::DecimalsFormatNotSupported {
                                value: a.to_string(),
                            }),
                        }
                    } else {
                        // element is not a number, this is an error
                        Err(ErrorCompanion::DecimalsFormatNotSupported {
                            value: a.to_string(),
                        })
                    }
                } else {
                    // decimals are an array with more than one element
                    Err(ErrorCompanion::DecimalsFormatNotSupported {
                        value: a.to_string(),
                    })
                }
            }

            // unexpected decimals format
            _ => Err(ErrorCompanion::DecimalsFormatNotSupported {
                value: a.to_string(),
            }),
        },

        // decimals are missing
        None => Err(ErrorCompanion::NoDecimals),
    }
}

fn unit(x: &Map<String, Value>) -> Result<String, ErrorCompanion> {
    match x.get("tokenSymbol") {
        // unit info is fetched in `system_properties` rpc call
        Some(a) => match a {
            // fetched unit value is a `String`
            Value::String(b) => {
                // definitive unit found
                Ok(b.to_string())
            }

            // fetched an array of units
            Value::Array(b) => {
                // array with a single element
                if b.len() == 1 {
                    // single `String` element array, process same as `String`
                    if let Value::String(c) = &b[0] {
                        // definitive unit found
                        Ok(c.to_string())
                    } else {
                        // element is not a `String`, this is an error
                        Err(ErrorCompanion::UnitFormatNotSupported {
                            value: a.to_string(),
                        })
                    }
                } else {
                    // units are an array with more than one element
                    Err(ErrorCompanion::UnitFormatNotSupported {
                        value: a.to_string(),
                    })
                }
            }

            // unexpected unit format
            _ => Err(ErrorCompanion::UnitFormatNotSupported {
                value: a.to_string(),
            }),
        },

        // unit missing
        None => Err(ErrorCompanion::NoUnit),
    }
}

pub fn system_properties_to_short_specs(
    system_properties: &Map<String, Value>,
    metadata: &RuntimeMetadataV15,
) -> Result<ShortSpecs, ErrorCompanion> {
    let optional_prefix_from_meta = optional_prefix_from_meta(metadata);
    let base58prefix = base58prefix(system_properties, optional_prefix_from_meta)?;
    let decimals = decimals(system_properties)?;
    let unit = unit(system_properties)?;
    Ok(ShortSpecs {
        base58prefix,
        decimals,
        unit,
    })
}
