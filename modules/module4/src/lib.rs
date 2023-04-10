#![allow(unused_imports)]
// use inteconnet::serdes::*;
use k256::schnorr::{
    signature::{Signer, Verifier},
    SigningKey, VerifyingKey,
};
use rand_core::OsRng; // requires 'getrandom' featureuse libinteronnect::serdes::*;
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
