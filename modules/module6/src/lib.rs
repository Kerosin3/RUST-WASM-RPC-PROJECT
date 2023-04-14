use k256::ecdsa::signature::Verifier;
use k256::ecdsa::{Signature, VerifyingKey};
use libinteronnect::serdes::{Operation, StatusFromWasm, WasmDataRecv, WasmDataSend};
use wapc_codec::messagepack::{deserialize, serialize};
use wapc_guest as wapc;
#[no_mangle]
pub fn wapc_init() {
    wapc::register_function("verify_message", verify_message);
}

fn verify_message(msg: &[u8]) -> wapc::CallResult {
    wapc::console_log(&String::from(
        "IN_WASM: Received request for `verify_message`: MODULE 6",
    ));
    let inputstruct: WasmDataSend = deserialize(msg)?; // deser Name
    let bad_msg = WasmDataRecv {
        payload_back: "Error".to_string(),
        status: StatusFromWasm::Error,
    };
    let bytes_bad = serialize(bad_msg)?;

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
            let Ok(restored_signed_message) = Signature::from_der(&restored_signed_message) else {
                let _resbad =
                    wapc::host_call("binding", "sample:namespace", "serdes_and_hash", &bytes_bad)?;
                return Ok(bytes_bad.to_vec());
            };
            let Ok(ver_key ) = VerifyingKey::from_sec1_bytes(&encoded_vkey) else {
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
                    status: StatusFromWasm::NotValid,
                };
                let bytes = serialize(msg_back)?;
                let _res =
                    wapc::host_call("binding", "sample:namespace", "serdes_and_hash", &bytes)?;
                Ok(bytes.to_vec())
            } else {
                let msg_back = WasmDataRecv {
                    payload_back: "Ok".to_string(),
                    status: StatusFromWasm::NotValid,
                };
                let bytes = serialize(msg_back)?;
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
            let bytes = serialize(msg_back)?;
            let _res = wapc::host_call("binding", "sample:namespace", "serdes_and_hash", &bytes)?;
            Ok(bytes.to_vec())
        }
    }
}
