#![allow(unused_imports)]
// use inteconnet::serdes::*;

use k256::schnorr::{
    signature::{Signer, Verifier},
    Signature, SigningKey, VerifyingKey,
};
use libinteronnect::serdes::*;
use serde::{Deserialize, Serialize};
use wapc_codec::messagepack::{deserialize, serialize};
use wapc_guest as wapc;
#[no_mangle]
pub fn wapc_init() {
    wapc::register_function("verify_message", verify_message);
}

//just return hardcoded
fn verify_message(msg: &[u8]) -> wapc::CallResult {
    wapc::console_log(&String::from(
        "IN_WASM: Received request for `verify_message`: MODULE 4",
    ));
    let inputstruct: WasmDataSend = deserialize(msg)?; // deser Name
    let bad_msg = WasmDataRecv {
        payload_back: "Error".to_string(),
        status: StatusFromWasm::Error,
    };
    let bytes_bad = serialize(&bad_msg)?;

    match inputstruct.oper {
        Operation::One => {
            let encoded_signed_msg = inputstruct.smessage;
            let encoded_vkey = inputstruct.vkey;
            let _testmessage = inputstruct.rmessage;
            wapc::console_log(&format!(
                "\nsmessage passed to wasm\n[{}]\nvkey:\n[{}]\nmessage:[{}]\n",
                &encoded_signed_msg,
                hex::encode(&encoded_vkey),
                &_testmessage
            ));

            let Ok(restored_signed_message) = hex::decode(&encoded_signed_msg) else {
                let _resbad =
                    wapc::host_call("binding", "sample:namespace", "serdes_and_hash", &bytes_bad)?;
                return Ok(bytes_bad.to_vec());
            };
            let Ok(restored_signed_message) = Signature::try_from(&restored_signed_message[..]) else {
                let _resbad =
                    wapc::host_call("binding", "sample:namespace", "serdes_and_hash", &bytes_bad)?;
                return Ok(bytes_bad.to_vec());
            };
            let Ok(ver_key ) = VerifyingKey::from_bytes(&encoded_vkey) else {
                let _resbad =
                    wapc::host_call("binding", "sample:namespace", "serdes_and_hash", &bytes_bad)?;
                return Ok(bytes_bad.to_vec());
            };
            if ver_key
                .verify(_testmessage.as_bytes(), &restored_signed_message)
                .is_ok()
            {
                let msg_back = WasmDataRecv {
                    payload_back: "Ok".to_string(),
                    status: StatusFromWasm::Valid,
                };
                let bytes = serialize(&msg_back)?;
                let _res =
                    wapc::host_call("binding", "sample:namespace", "serdes_and_hash", &bytes)?;
                Ok(bytes.to_vec())
            } else {
                let msg_back = WasmDataRecv {
                    payload_back: "Ok".to_string(),
                    status: StatusFromWasm::NotValid,
                };
                let bytes = serialize(&msg_back)?;
                let _res =
                    wapc::host_call("binding", "sample:namespace", "serdes_and_hash", &bytes)?;
                Ok(bytes.to_vec())
            }
        }
        _ => {
            let msg_back = WasmDataRecv {
                payload_back: "11111".to_string(),
                status: StatusFromWasm::Error,
            };
            let bytes = serialize(&msg_back)?;
            let _res = wapc::host_call("binding", "sample:namespace", "serdes_and_hash", &bytes)?;
            Ok(bytes.to_vec())
        }
    }
}
