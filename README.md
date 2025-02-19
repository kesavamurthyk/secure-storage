# Secure Storage 🔒

![Rust](https://img.shields.io/badge/Rust-%23dea584?style=for-the-badge&logo=rust&logoColor=white)
![WebAssembly](https://img.shields.io/badge/WebAssembly-%237864BE?style=for-the-badge&logo=webassembly&logoColor=white)
![ChaCha20-Poly1305](https://img.shields.io/badge/Encryption-ChaCha20--Poly1305-blueviolet)

## Overview
**Secure Storage 🔒** is a WebAssembly (WASM) module written in Rust that provides secure, high-performance encryption and decryption capabilities using the **ChaCha20-Poly1305** AEAD (Authenticated Encryption with Associated Data) algorithm. This project is ideal for securely storing and transmitting sensitive data in web applications.

## Features
1. ✅ **End-to-end Encryption** - Uses the ChaCha20-Poly1305 cipher for secure data encryption.
2. ✅ **WASM Powered** - High-performance cryptographic operations in the browser.
3. ✅ **Lightweight & Fast** - Rust’s efficiency ensures minimal overhead.
4. ✅ **Authenticated Encryption** - Ensures data integrity and authenticity.

## Installation
Ensure you have Rust and `wasm-pack` installed:

```sh
cargo install wasm-pack
```

Clone the repository and build the WASM package:

```sh
git clone https://github.com/yourusername/secure-storage.git
cd secure-storage
```
### Generate package
```Sh
wasm-pack build --target web 
```

## Usage
### Importing in JavaScript
After building, you can use the WASM module in your JavaScript project:

```js
import init, { set_item, get_item } from './pkg/secure_storage.js';

async function run() {
    await init();
    const data = {} // storage payload
    set_item("data", JSON.stringify(data))
    console.log(get_item("data"))
}

run();
```

## API
#### `set_item(key: string, data: string) => void`
Generates and sets the encrypted data in the localstorage

#### `get_item(key: string) => string`
Gets the decrypted data in the localstorage

#### `remove(key: string) => void`
Removes the key from localstorage

#### `clear() => void`
Clears the localstorage

## Contributing
We welcome contributions! Feel free to submit issues, feature requests, or pull requests.

1. Fork the repo
2. Create a new branch: `git checkout -b feature-branch`
3. Commit changes: `git commit -m "Add a new feature"`
4. Push to the branch: `git push origin feature-branch`
5. Create a Pull Request

## License
This project is licensed under the MIT License.

## Acknowledgments
Special thanks to the Rust and WebAssembly communities for their amazing tools and documentation!

---

🚀 **Secure your data with Secure Storage 🔒 today!**
