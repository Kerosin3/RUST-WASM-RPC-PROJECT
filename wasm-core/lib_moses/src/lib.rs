#![allow(dead_code)]
pub mod wasm_lib {
    extern crate anyhow;
    extern crate wapc;
    extern crate wasmtime_runner;
    use self::anyhow::Result;
    use wasm_lib::wapc::*;
    use wasm_lib::wasmtime_runner::*;
    //--------------------------------------------------------
    pub struct HostProvider {
        host: WapcHost,
    }
    impl HostProvider {
        pub fn assign(engine: Engine) -> Result<HostProvider> {
            Ok(Self {
                host: WapcHost::new(engine.internal, Some(Box::new(host_callback)))?,
            })
        }
        pub fn execute_func_call(&self, func_name: &str, ser_data: &[u8]) -> Result<Vec<u8>> {
            Ok(self.host.call(func_name, ser_data)?)
        }
        pub fn execute_replace_module(&self, mod_bytes: Vec<u8>) -> Result<()> {
            self.host.replace_module(&mod_bytes)?;
            Ok(())
        }
    }
    //     use wasm_lib::wasmtime_runner::*;
    pub struct Engine {
        internal: Box<WasmtimeEngineProvider>,
    }
    impl Engine {
        pub fn new(module_bytes: Vec<u8>) -> Self {
            Self {
                internal: Box::new(
                    WasmtimeEngineProviderBuilder::new()
                        .module_bytes(&module_bytes)
                        .build()
                        .unwrap(),
                ),
            }
        }
    }
    fn host_callback(
        id: u64,
        bd: &str,
        ns: &str,
        op: &str,
        payload: &[u8],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        println!(
            "Guest {} invoked '{}->{}:{}' on the host with a payload of '{}'",
            id,
            bd,
            ns,
            op,
            hex::encode(payload)
        );
        Ok(vec![])
    }
}
pub mod serdes {

    extern crate serde;
    use self::serde::{Deserialize, Serialize};
    //simple struct to pass to wasm module and calc hash inside
    #[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
    pub struct PersonSend {
        pub first_name: String,
    }
    // recv struct
    #[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
    pub struct PersonHashedRecv {
        pub first_name: String,
        pub hash: u64,
    }
}
