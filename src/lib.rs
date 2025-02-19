mod utils;
mod xchacha20poly1305;
extern crate web_sys;

use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use web_sys::{window, Storage, console};
use web_sys::js_sys::Boolean;
use xchacha20poly1305::XChaCha20Poly1305Wrapper;
// use lazy_static::lazy_static;
// use serde_json::{Value, json};

// lazy_static! {
//     static ref CONFIG_VALUE: Mutex<Value> = Mutex::new(json!({})); // Default empty JSON object
// }

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

fn get_storage() -> Storage {
    window()
        .expect("window not found")
        .local_storage()
        .expect("Failed to access localStorage")
        .expect("localStorage not detected")
}

#[wasm_bindgen(js_name="setItem")]
pub fn set_item(key: &str, data: &str, encrypted: Option<bool>) {
    let mut should_encrypt = encrypted.unwrap_or(true);
    let storage = get_storage();
    if should_encrypt {
        let chacha20 = XChaCha20Poly1305Wrapper::new();
        match chacha20.encrypt_data(data.as_bytes()) {
            Ok(encrypted_data) => storage.set_item(key, &encrypted_data).expect("Failed to store encrypted data"),
            Err(e) => {
                console::error_1(&JsValue::from_str(e));
            }
        }
    } else {
        storage.set_item(key, &data).expect("Failed to store encrypted data")
    }
}

#[wasm_bindgen(js_name="getItem")]
pub fn get_item(key: &str) -> String {
    let storage = get_storage();
    let chacha20 = XChaCha20Poly1305Wrapper::new();

    if let Some(data) = storage.get_item(key).expect("Failed to retrieve data") {
        if !data.contains("<C>") {
            return data;
        }
        return match chacha20.decrypt_data(&data) {
            Ok(decrypted_data) => decrypted_data,
            Err(e) => {
                console::error_1(&JsValue::from_str(e));
                return "".to_string();
            }
        }
    }
    "".to_string()
}

#[wasm_bindgen(js_name="removeItem")]
pub fn remove_item(key: &str) {
    let storage = get_storage();

    storage.remove_item(key).expect("Failed to remove data");

}

#[wasm_bindgen(js_name="clear")]
pub fn clear() {
    let storage = get_storage();
    storage.clear().expect("Failed to clear data");
}

// #[wasm_bindgen]
// pub fn set_config(json_str: String) {
//     match serde_json::from_str::<Value>(&json_str) {
//         Ok(parsed_json) => {
//             let mut config = CONFIG_VALUE.lock().unwrap();
//             *config = parsed_json;
//         }
//         Err(_) => console::error_1(&JsValue::from("Invalid configuration")),
//     }
// }
//
// #[wasm_bindgen]
// pub fn get_config() -> String {
//     let config = CONFIG_VALUE.lock().unwrap();
//     serde_json::to_string(&*config).unwrap()
// }
