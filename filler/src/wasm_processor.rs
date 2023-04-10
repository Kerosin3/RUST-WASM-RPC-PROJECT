pub mod implement {
    use base64::{engine::general_purpose, Engine as _};
    use crossbeam_channel::unbounded;
    use k256::schnorr::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    };
    use libinteronnect::serdes::*;
    use libmoses::wasm_lib::{Engine, HostProvider};
    use libshmem::datastructs::*;
    use std::path::Path;
    use std::time::Instant;
    use wapc_codec::messagepack::{deserialize, serialize};

    pub fn process_in_wasm(
        recv_sig_msg: crossbeam_channel::Receiver<String>,
        recv_ver_key: crossbeam_channel::Receiver<Vec<u8>>,
    ) -> Result<(), wapc::errors::Error> {
        let mut store_signed_msg: Vec<String> = vec![];
        let mut store_ver_keys: Vec<Vec<u8>> = vec![];
        for _ms in 0..MESSAGES_NUMBER {
            store_signed_msg.push(recv_sig_msg.recv().unwrap()); //values
            store_ver_keys.push(recv_ver_key.recv().unwrap()); // keys
        }
        println!("Starting demo");
        let root_path = project_root::get_project_root().unwrap();
        let module1 = Path::new(&root_path)
            .join("target")
            .join("wasm32-unknown-unknown")
            .join("debug")
            .join("module3_hash.wasm");
        let module_bytes1 = std::fs::read(module1)
            .expect("WASM module 1 could not be read, run example from wasmtime-provider folder"); // read module 1
        let func = "serdes_example".to_string();
        let engine = Engine::new(module_bytes1); // load engine
        let host = HostProvider::assign(engine).unwrap();
        let now = Instant::now();
        for _i in 0..MESSAGES_NUMBER {
            let s_msg = store_signed_msg.pop().unwrap();
            let mut ver_key = store_ver_keys.pop().unwrap();
            ver_key.truncate(SIGN_SIZE);
            let restored_ver_key = VerifyingKey::from_bytes(&ver_key).unwrap(); // RESTORE KEY
            println!(
                "signed message is [{}], ver key is {}",
                &s_msg,
                hex::encode(ver_key)
            );
            let restored_signed_message = general_purpose::STANDARD.decode(&s_msg).unwrap();
            //             let signed_msg_r: Signature =
            //                 unsafe { std::ptr::read(restored_signed_message.as_ptr() as *const _) };
            //             assert!(restored_ver_key.verify(b"haha", &signed_msg_r).is_ok());
            /*            println!(
                "=====>DECODED: MSG {:?} V_KEY:{:?}",
                restored_value, restored_ver_key
            );*/

            // supply person struct
            let person = StructSend {
                payload: s_msg.clone(),
                id: 0,
                oper: Operation::Two,
            };
            let serbytes: Vec<u8> = serialize(&person).unwrap(); // serialize
                                                                 //             let encoded = hex::encode(serbytes.clone()); // examine
                                                                 //             println!("serialized message: {}", encoded);
            println!("calling wasm guest function to process text [{}]", s_msg);
            println!("---------------CALLING MAIN MODULE------------------");
            let result = host.execute_func_call(&func, &serbytes).unwrap();
            let recv_struct: StructRecv = deserialize(&result).unwrap();
            println!("Deserialized : {:?}", recv_struct);
        }
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        Ok(())
    }
}
