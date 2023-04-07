#![allow(unused_imports)]
// use inteconnet::serdes::*;
use aes_gcm::{
    aead::{heapless::Vec, AeadInPlace, KeyInit, OsRng},
    Aes256Gcm,
    Nonce, // Or `Aes128Gcm`
};
use libinteronnect::serdes::*;
use serde::{Deserialize, Serialize};
use wapc_codec::messagepack::{deserialize, serialize};
use wapc_guest as wapc;
#[no_mangle]
pub fn wapc_init() {
    wapc::register_function("serdes_example", serdes_example);
}
//just return hardcoded
fn serdes_example(msg: &[u8]) -> wapc::CallResult {
    wapc::console_log(&String::from(
        "IN_WASM: Received request for `serdes_and_hash`: MODULE 3",
    ));
    let inputstruct: StructSend = deserialize(msg)?; // deser Name
    match inputstruct.oper {
        Operation::One => {
            let key = Aes256Gcm::generate_key(&mut OsRng);
            let cipher = Aes256Gcm::new(&key);
            let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

            let mut buffer: Vec<u8, 128> = Vec::new(); // Note: buffer needs 16-bytes overhead for auth tag tag
            buffer.extend_from_slice(b"plaintext message");

            // Encrypt `buffer` in-place, replacing the plaintext contents with ciphertext
            cipher.encrypt_in_place(nonce, b"", &mut buffer).unwrap();
            let msg_back = StructRecv {
                payload_back: "Heheh".to_string(),
            };
            let bytes = serialize(&msg_back)?;
            let _res = wapc::host_call("binding", "sample:namespace", "serdes_and_hash", &bytes)?;
            Ok(bytes.to_vec())
        }
        _ => {
            let msg_back = StructRecv {
                payload_back: "11111".to_string(),
            };
            let bytes = serialize(&msg_back)?;
            let _res = wapc::host_call("binding", "sample:namespace", "serdes_and_hash", &bytes)?;
            Ok(bytes.to_vec())
        }
    }
}
