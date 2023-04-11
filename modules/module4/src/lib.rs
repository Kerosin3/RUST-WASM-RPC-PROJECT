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
    wapc::register_function("serdes_example", serdes_example);
}

//just return hardcoded
fn serdes_example(msg: &[u8]) -> wapc::CallResult {
    wapc::console_log(&String::from(
        "IN_WASM: Received request for `serdes_and_hash`: MODULE 4",
    ));
    let inputstruct: WasmDataSend = deserialize(msg)?; // deser Name
    let bad_msg = StructRecv {
        payload_back: "Error".to_string(),
    };
    let bytes_bad = serialize(&bad_msg)?;
    let msg_back = StructRecv {
        payload_back: "default".to_string(),
    };

    match inputstruct.oper {
        Operation::One => {
            let encoded_signed_msg = inputstruct.smessage;
            let encoded_vkey = inputstruct.vkey;
            let _testmessage = inputstruct.rmessage;

            if hex::decode(&encoded_signed_msg).is_err() {
                let _resbad =
                    wapc::host_call("binding", "sample:namespace", "serdes_and_hash", &bytes_bad)?;
                return Ok(bytes_bad.to_vec());
            }
            //decode gonna be ok
            let restored_signed_message = hex::decode(&encoded_signed_msg).unwrap();

            if Signature::try_from(&restored_signed_message[..]).is_err() {
                let _resbad =
                    wapc::host_call("binding", "sample:namespace", "serdes_and_hash", &bytes_bad)?;
                return Ok(bytes_bad.to_vec());
            }
            let restored_signed_message =
                Signature::try_from(&restored_signed_message[..]).unwrap();
            if VerifyingKey::from_bytes(&encoded_vkey).is_err() {
                let _resbad =
                    wapc::host_call("binding", "sample:namespace", "serdes_and_hash", &bytes_bad)?;
                return Ok(bytes_bad.to_vec());
            }
            let ver_key = VerifyingKey::from_bytes(&encoded_vkey).unwrap();
            if ver_key
                .verify(_testmessage.as_bytes(), &restored_signed_message)
                .is_ok()
            {
                let msg_back = StructRecv {
                    payload_back: "OK".to_string(),
                };
                let bytes = serialize(&msg_back)?;
                let _res =
                    wapc::host_call("binding", "sample:namespace", "serdes_and_hash", &bytes)?;
                Ok(bytes.to_vec())
            } else {
                let bytes = serialize(&msg_back)?;
                let _res =
                    wapc::host_call("binding", "sample:namespace", "serdes_and_hash", &bytes)?;
                Ok(bytes.to_vec())
            }
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
