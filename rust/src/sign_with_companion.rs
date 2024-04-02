use parity_scale_codec::Encode;
use std::sync::Arc;

use crate::definitions::TransferData;

#[uniffi::export(with_foreign)]
pub trait SignByCompanion: Send + Sync + std::fmt::Debug {
    fn make_signature(&self, data: Vec<u8>) -> Vec<u8>;
    fn export_public_key(&self) -> Vec<u8>;
}

#[derive(uniffi::Object)]
pub struct SignatureMaker {
    pub signature_maker: Arc<dyn SignByCompanion>,
}

#[uniffi::export]
impl SignatureMaker {
    #[uniffi::constructor]
    pub fn new(signature_maker: Arc<dyn SignByCompanion>) -> Self {
        Self { signature_maker }
    }

    #[uniffi::method]
    pub fn signed_data(&self, encoded_data: Vec<u8>) -> Vec<u8> {
        let companion_signature = self.signature_maker.make_signature(encoded_data.to_owned());
        let companion_public_key = self.signature_maker.export_public_key();
        TransferData {
            encoded_data,
            companion_signature,
            companion_public_key,
        }
        .encode()
    }
}
