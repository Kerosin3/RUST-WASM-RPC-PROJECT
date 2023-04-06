use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use wapc_codec::messagepack::{deserialize, serialize};

//simple struct to pass to wasm module and calc hash inside
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
struct PersonSend {
    first_name: String,
}
// recv struct
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
struct PersonHashedRecv {
    first_name: String,
    hash: u64,
}
use libmoses::wasm_lib::{Engine, HostProvider};
use std::path::Path;
use wapc::WapcHost;
use wasmtime_runner::WasmtimeEngineProviderBuilder;
pub fn main() -> Result<(), wapc::errors::Error> {
    env_logger::init();
    println!("Starting demo");
    let root_path = project_root::get_project_root().unwrap();
    let module1 = Path::new(&root_path)
        .join("modules")
        .join("module1")
        .join("build")
        .join("module1_hash.wasm");
    let module2 = Path::new(&root_path)
        .join("modules")
        .join("module2")
        .join("build")
        .join("module2_hash.wasm");

    let name = &std::env::args().nth(1).expect("pass some name to serde");
    let module_bytes1 = std::fs::read(module1)
        .expect("WASM module 1 could not be read, run example from wasmtime-provider folder"); // read module 1
    let module_bytes2 = std::fs::read(module2)
        .expect("WASM module 2 could not be read, run example from wasmtime-provider folder"); // read module 2
    let func = "serdes_example".to_string();
    let engine = Engine::new(module_bytes1.to_owned()); // load engine
    assert_ne!(module_bytes1, module_bytes2); // test modules binaries not equal
    let host = HostProvider::assign(engine).unwrap();
    println!("Calling guest (wasm) function: {}", func);
    // supply person struct
    let person = PersonSend {
        first_name: name.clone(),
    };
    let serbytes: Vec<u8> = serialize(&person).unwrap(); // serialize
    let encoded = hex::encode(serbytes.clone()); // examine
    println!("serialized message: {}", encoded);
    println!("calling wasm guest function to process text [{}]", name);
    println!("---------------CALLING MAIN MODULE------------------");
    let result = host.execute_func_call(&func, &serbytes).unwrap();
    let recv_struct: PersonHashedRecv = deserialize(&result).unwrap();
    println!("Deserialized : {:?}", recv_struct);
    println!("---------------REPLACING MODULE------------------");
    host.execute_replace_module(module_bytes2).unwrap(); // hotswapping
    let serbytes2: Vec<u8> = serialize(&person).unwrap();
    let encoded2 = hex::encode(serbytes2.clone());
    println!("serialized message: {}", encoded2);
    println!("calling wasm guest function to process text [{}]", name);
    println!("Calling guest (wasm) function: {}", func);
    let res2 = host
        .execute_func_call("serdes_example", &serbytes2)
        .unwrap(); //calling
    let recv_struct2: PersonHashedRecv = deserialize(&res2).unwrap();
    println!("Deserialized : {:?}", recv_struct2);
    assert_ne!(recv_struct, recv_struct2);
    Ok(())
}
