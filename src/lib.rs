mod utils;
mod xchacha20poly1305;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use web_sys::{window, Storage, console};
use base64::{engine::general_purpose, Engine as _};
use xchacha20poly1305::XChaCha20Poly1305Wrapper;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

fn get_storage() -> Option<Storage> {
    window()?.local_storage().ok()?
}

/// Stores an item in local storage with optional encryption.
#[wasm_bindgen(js_name="setItem")]
pub fn set_item(key: &str, data: &str, encrypted: Option<bool>) {
    if let Some(storage) = get_storage() {
        if encrypted.unwrap_or(true) {
            let chacha20 = XChaCha20Poly1305Wrapper::new();
            match chacha20.encrypt_data(data.as_bytes()) {
                Ok(encrypted_data) => {
                    if let Err(e) = storage.set_item(key, &encrypted_data) {
                        console::error_1(&JsValue::from_str(&format!("Storage error: {}", e.as_string().unwrap().to_string())));
                    }
                }
                Err(e) => console::error_1(&JsValue::from_str(&format!("Encryption error: {}", e))),
            }
        } else if let Err(e) = storage.set_item(key, data) {
            console::error_1(&JsValue::from_str(&format!("Storage error: {}", e.as_string().unwrap().to_string())));
        }
    }
}

/// Retrieves an item from local storage and decrypts it if necessary.
#[wasm_bindgen(js_name="getItem")]
pub fn get_item(key: &str) -> String {
    let Some(storage) = get_storage() else { return String::new(); };

    if let Some(data) = storage.get_item(key).ok().flatten() {
        // If the stored data is not encrypted, return it directly.

        if !data.contains("<C>") {
            return data;
        }

        if let Some((prefix, rest)) = data.split_once("_") {
            let decoded_buffer = general_purpose::STANDARD.decode(&rest).unwrap();
            let decoded_data = decoded_buffer.into_iter().map(|x| x as char).collect::<String>();

            let chacha20 = XChaCha20Poly1305Wrapper::new();
            chacha20.decrypt_data(&decoded_data).unwrap_or_else(|e| {
                console::error_1(&JsValue::from_str(&format!("Decryption error: {}", e)));
                String::new()
            })
        } else {
            String::new()
        }

    } else {
        String::new()
    }
}

/// Removes an item from local storage.
#[wasm_bindgen(js_name="removeItem")]
pub fn remove_item(key: &str) {
    if let Some(storage) = get_storage() {
        if let Err(e) = storage.remove_item(key) {
            console::error_1(&JsValue::from_str(&format!("Failed to remove item: {}", e.as_string().unwrap().to_string())));
        }
    }
}

/// Clears all stored data in local storage.
#[wasm_bindgen(js_name="clear")]
pub fn clear() {
    if let Some(storage) = get_storage() {
        if let Err(e) = storage.clear() {
            console::error_1(&JsValue::from_str(&format!("Failed to clear storage: {}", e.as_string().unwrap().to_string())));
        }
    }
}
