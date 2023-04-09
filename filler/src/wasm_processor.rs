pub mod implement {
    use crossbeam_channel::unbounded;
    use libinteronnect::serdes::*;
    use libmoses::wasm_lib::{Engine, HostProvider};
    use libshmem::datastructs::*;
    use std::path::Path;
    use std::time::Instant;
    use wapc_codec::messagepack::{deserialize, serialize};

    pub fn process_in_wasm(
        recv: crossbeam_channel::Receiver<String>,
    ) -> Result<(), wapc::errors::Error> {
        let mut store: Vec<String> = vec![];
        for ms in 0..MESSAGES_NUMBER {
            store.push(recv.recv().unwrap());
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
            let name = store.pop().unwrap();
            println!("Calling guest (wasm) function: {}", func);
            println!("----> name is {}", name);
            // supply person struct
            let person = StructSend {
                payload: name.clone(),
                id: 0,
                oper: Operation::Two,
            };
            let serbytes: Vec<u8> = serialize(&person).unwrap(); // serialize
            let encoded = hex::encode(serbytes.clone()); // examine
            println!("serialized message: {}", encoded);
            println!("calling wasm guest function to process text [{}]", name);
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
