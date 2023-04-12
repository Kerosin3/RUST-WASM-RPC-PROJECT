pub mod implement {
    //################################################3
    //################################################3
    //################################################3
    const FUNC_WASM_NAME: &str = "verify_message";
    const MSG_LIMIT: usize = 128;
    //################################################3
    //################################################3
    //################################################3
    use crate::native_verification::implement::*;
    use crossbeam_channel::unbounded;
    use k256::schnorr::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    };
    use libinteronnect::serdes::*;
    use libmoses::wasm_lib::{Engine, HostProvider};
    use libshmem::datastructs::*;
    use native_verification::implement::test_validity;
    use std::io::{Error, ErrorKind};
    use std::path::Path;
    use std::time::Instant;
    use wapc_codec::messagepack::{deserialize, serialize};

    use console::Style;

    use crate::native_verification;
    pub fn process_in_wasmtime_with_replacing(
        recv_sig_msg: crossbeam_channel::Receiver<String>,
        recv_ver_key: crossbeam_channel::Receiver<Vec<u8>>,
        right_messages: Vec<String>,
    ) -> Result<(), wapc::errors::Error> {
        let yellow = Style::new().yellow();
        let magenta = Style::new().magenta();
        let red = Style::new().red();
        let mut right_messages: Vec<String> = right_messages.into_iter().rev().collect(); // overkill
        let mut store_signed_msg: Vec<String> = vec![];
        let mut store_ver_keys: Vec<Vec<u8>> = vec![];
        for _ms in 0..MESSAGES_NUMBER {
            store_signed_msg.push(recv_sig_msg.recv().unwrap()); //values
            store_ver_keys.push(recv_ver_key.recv().unwrap()); // keys
        }
        println!(
            "{}",
            red.apply_to("START WASM PROCESSING USING WASMTIME RUNNER WITH REPLACE")
        );
        let root_path = project_root::get_project_root().unwrap();
        let module1 = Path::new(&root_path)
            .join("target")
            .join("wasm32-unknown-unknown")
            .join("release")
            .join("module4_verify.wasm");
        let module_bytes1 = std::fs::read(module1).expect("WASM module 1 could not be read "); // read module 1
        let module2 = Path::new(&root_path)
            .join("target")
            .join("wasm32-unknown-unknown")
            .join("release")
            .join("module5_verify.wasm");
        let module_bytes2 = std::fs::read(module2).expect("WASM module 2 could not be read"); // read module 2

        let func = FUNC_WASM_NAME;
        let engine = Engine::new(module_bytes1); // load engine
        let host = HostProvider::assign(engine).unwrap();
        let now = Instant::now();
        let mut valid_n: usize = 0;
        for _i in 0..MESSAGES_NUMBER {
            let mut s_msg = store_signed_msg.pop().unwrap();
            /*if _i == 1 {
                s_msg.replace_range(0..1, "x"); // error handling
            }*/
            s_msg.truncate(MSG_LIMIT); // oh shi
            let mut ver_key = store_ver_keys.pop().unwrap();
            ver_key.truncate(SIGN_SIZE);
            let rmsg = right_messages.pop().unwrap().as_str().to_string();
            println!(
                "[{}]\nsigned message is [{}]\nver key is {}\nmessage:{}",
                _i,
                yellow.apply_to(&s_msg),
                magenta.apply_to(hex::encode(&ver_key)),
                &rmsg
            );
            let data_to_wasm = WasmDataSend {
                rmessage: rmsg.to_string(),
                vkey: ver_key,
                smessage: s_msg.to_owned(),
                id: _i as i32,
                oper: Operation::One,
            };
            let serbytes: Vec<u8> = serialize(&data_to_wasm).unwrap(); // serialize
            println!("{}", yellow.apply_to("CALLING WASM MODULE"));
            //####################REPLACING MODULE#########################
            if _i == MESSAGES_NUMBER / 2 {
                //replace module
                println!(
                    "{}",
                    red.apply_to("xxxxxxxxxxxxxxx----replacing module---xxxxxxxxxxxxxxx")
                );
                host.execute_replace_module(&module_bytes2).unwrap();
            }
            let result = host.execute_func_call(&func, &serbytes).unwrap();
            let recv_struct: WasmDataRecv = deserialize(&result).unwrap();
            let whether_valid = recv_struct.status;
            println!("Valivation from WASM: {:?}", whether_valid);
            if whether_valid == StatusFromWasm::Valid {
                valid_n += 1;
            }
        }
        let elapsed = now.elapsed();
        println!(
            "valid messages {}, total processed messages: {}",
            valid_n, MESSAGES_NUMBER
        );
        println!("Elapsed: >>>>{:.2?}<<<<", elapsed);
        Ok(())
    }
}