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
        pub fn execute_replace_module(&self, mod_bytes: &[u8]) -> Result<()> {
            self.host.replace_module(mod_bytes)?;
            Ok(())
        }
    }
    pub struct Engine {
        internal: Box<WasmtimeEngineProvider>,
    }
    impl Engine {
        pub fn new(module_bytes: &[u8]) -> Self {
            Self {
                internal: Box::new(
                    WasmtimeEngineProviderBuilder::new()
                        .module_bytes(module_bytes)
                        .build()
                        .unwrap(),
                ),
            }
        }
    }
    pub fn host_callback(
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
