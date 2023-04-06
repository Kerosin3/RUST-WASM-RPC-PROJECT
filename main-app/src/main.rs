use libinteronnect::serdes::*;
use libmoses::wasm_lib::{Engine, HostProvider};
use log::{debug, error, info, warn};
use std::path::Path;
use wapc::WapcHost;
use wapc_codec::messagepack::{deserialize, serialize};
use wasmtime_runner::WasmtimeEngineProviderBuilder;

pub fn main() -> Result<(), wapc::errors::Error> {
    env_logger::init();
    println!("Starting demo");
    let root_path = project_root::get_project_root().unwrap();
    let module1 = Path::new(&root_path)
        .join("target")
        .join("wasm32-unknown-unknown")
        .join("debug")
        .join("module3_hash.wasm");
    let name = &std::env::args().nth(1).expect("pass some name to serde");
    let module_bytes1 = std::fs::read(module1)
        .expect("WASM module 1 could not be read, run example from wasmtime-provider folder"); // read module 1
    let func = "serdes_example".to_string();
    let engine = Engine::new(module_bytes1.to_owned()); // load engine
    let host = HostProvider::assign(engine).unwrap();
    println!("Calling guest (wasm) function: {}", func);
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
    Ok(())
}
