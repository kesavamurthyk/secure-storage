use std::convert::TryInto;
use chacha20poly1305::{aead::{Aead, KeyInit, OsRng}, AeadCore, XChaCha20Poly1305, XNonce};
use base64::{engine::general_purpose, Engine as _};
use chacha20poly1305::aead::generic_array::GenericArray;
use chacha20poly1305::aead::rand_core::{RngCore};
use wasm_bindgen::JsValue;

extern crate web_sys;

pub struct XChaCha20Poly1305Wrapper {
    encoder: general_purpose::GeneralPurpose,
    key: [u8; 32],
}

impl XChaCha20Poly1305Wrapper {
    pub fn new() -> Self {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        Self {
            key,
            encoder: general_purpose::STANDARD,
        }
    }

    fn get_nonce(&self) -> XNonce {
        XChaCha20Poly1305::generate_nonce(&mut OsRng)
    }

    fn vec_to_string(data: Vec<u8>) -> String {
        data.iter().map(|&x| x as char).collect::<String>()
    }

    pub fn encrypt_data(&self, data: &[u8]) -> Result<String, &'static str> {
        let cipher = XChaCha20Poly1305::new(&GenericArray::from(self.key));
        let gen_nonce = Self::get_nonce(&self);
        let nonce = XNonce::from_slice(&gen_nonce);

        let encrypted_data = cipher.encrypt(&nonce, data).map_err(|e| "Encryption failed")?;
        let encoded_data = self.encoder.encode::<&Vec<u8>>(&encrypted_data.as_ref());
        let encoded_nonce = self.encoder.encode::<&[u8]>(nonce);
        let encoded_key = self.encoder.encode::<&[u8; 32]>(&self.key);

        Ok(format!("<C>_{}", self.encoder.encode(format!("{encoded_data}_{encoded_nonce}_{encoded_key}"))))
    }

    pub fn decrypt_data(&self, encrypted: &str) -> Result<String, &'static str> {
        let parts: Vec<&str> = encrypted.split('_').collect();
        if parts.len() != 3 {
            use web_sys::console;
            console::error_1(&JsValue::from_str(&"Invalid Encryption"));
            return Ok("".to_string());
        }

        let mut encrypted_data = self.encoder.decode(parts[0]).map_err(|_| "Decoding failed")?;
        let nonce_bytes = self.encoder.decode(parts[1]).map_err(|_| "Decoding nonce failed")?;
        let key_bytes = self.encoder.decode(parts[2]).map_err(|_| "Decoding key_bytes failed")?;
        let arr: [u8; 32] = key_bytes.try_into().map_err(|_| "Invalid key")?;

        let cipher = XChaCha20Poly1305::new(&GenericArray::from(arr));

        let nonce = XNonce::from_slice(&nonce_bytes);

        let decrypted_data = cipher.decrypt(nonce, encrypted_data.as_ref()).map_err(|_| "Decoding failed")?;

        Ok(decrypted_data.into_iter().map(|b| b as char).collect())
    }
}
