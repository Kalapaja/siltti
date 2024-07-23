#![deny(unused_crate_dependencies)]
#![deny(rustdoc::broken_intra_doc_links)]

use rustls as _;

pub mod database;
pub mod definitions;
pub mod error;
pub mod fetch;
pub mod interface;
pub mod process_input;
pub mod qr_reader;
pub mod sign_with_companion;
#[cfg(test)]
mod tests;
pub mod utils;

uniffi::setup_scaffolding!();
