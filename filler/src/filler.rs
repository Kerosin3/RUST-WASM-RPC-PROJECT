#![allow(unused_imports)]
//#######################################################
//#######################################################
//#######################################################
// const KEY_LEN: usize = 10;
const _VAL_LEN: usize = 10;
const EXTRA_PRINT: bool = true;
//#######################################################
//#######################################################
//#######################################################
//-------------------------------------------------------
//-------------------------------------------------------
use log::*;
use std::io::stdin;
mod client_shmem;
use client_shmem::shmem_impl::*;
mod native_verification;
use libshmem::datastructs::*;
use native_verification::implement::*;
use redis::{
    from_redis_value,
    streams::{StreamRangeReply, StreamReadOptions, StreamReadReply},
    AsyncCommands, Client,
};
use transport::transport_interface_client::TransportInterfaceClient;
use transport::{ClientCommand, ClientRequest, ServerResponse, StatusMsg};
mod wasm_processor_wasmtime;
mod wasm_processor_wasmtime_module_replace;
use wasm_processor_wasmtime::implement::*;
mod wasm_processor_native;
use wasm_processor_native::implement_native::*;
mod wasm_processor_wasm3;
use wasm_processor_wasm3::implement_wasm3::*;
pub mod transport {
    tonic::include_proto!("transport_interface");
}
use console::Style;
use crossbeam_channel::unbounded;
use k256::schnorr::{
    signature::{Signer, Verifier},
    Signature, SignatureBytes, SigningKey, VerifyingKey,
};
use rand_core::OsRng;
use rnglib::{Language, RNG};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short = "r", long = "runner")]
    runner: u32,
}
//-------------------------------------------------------
//-------------------------------------------------------
//-------------------------------------------------------
pub enum Runner {
    Wasmtime,
    Wasm3,
    Native,
    Replace,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    let opt = match opt.runner {
        0 => Runner::Wasmtime,
        1 => Runner::Wasm3,
        2 => Runner::Native,
        3 => Runner::Replace,
        _ => Runner::Native,
    };
    env_logger::init();
    let cyan = Style::new().cyan();
    info!("starting client app");
    let mut client = TransportInterfaceClient::connect("http://[::1]:8080")
        .await
        .unwrap();
    let response = client
        .establish_connection(BaseRequest::construct(Cmd1::Establish))
        .await
        .unwrap();
    let init = response.into_inner();
    let redis_connection = Client::open("redis://127.0.0.1")?;
    let mut con = redis_connection.get_tokio_connection().await?;
    let mut right_messages: Vec<String> = Vec::new(); // storage
                                                      //------------------process PRC--------------------------------
    if let Some(t) = StatusMsg::from_i32(init.msg_status) {
        if t != StatusMsg::Proceed {
            error!("cannot connect to server");
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "fron initial connect",
            ))
            .into());
        }
        //--------------------------------------------------------
        info!(
            "starting generating random key-val messages pair, n=[{}]",
            MESSAGES_NUMBER
        );
        let rng = RNG::try_from(&Language::Roman).unwrap();
        for _i in 0..MESSAGES_NUMBER {
            // sending to server
            let signing_key = SigningKey::random(&mut OsRng); // generate sign key
            let verifying_key_bytes: [u8; 32] = signing_key
                .verifying_key()
                .to_bytes()
                .as_slice()
                .try_into()
                .expect("wrong length"); // 32-bytes VERIFY KEY
            let unique_key: Vec<u8> = verifying_key_bytes.into(); // verify key to vec
            let msg = rng.generate_name();
            println!("generating message: {} ", cyan.apply_to(&msg));
            right_messages.push(msg.to_owned());
            let signatured_msg = signing_key.sign(msg.as_bytes()).to_bytes(); // sign msg
            let signed_msg = hex::encode(signatured_msg); // encode signed
            if EXTRA_PRINT {
                println!(
                    "---[{}]---\nkey={}\nS_MESSAGE={:?}",
                    _i,
                    hex::encode(&unique_key), //ver key
                    signed_msg                // msg signed
                );
            }
            con.xadd("stream_storage", "*", &[(signed_msg.clone(), unique_key)])
                .await?;
        }
        // send answers OK, senting message to PRC
        let _response = client
            .establish_connection(BaseRequest::construct(Cmd1::Sending(
                "stream_storage".to_string(),
            )))
            .await
            .unwrap();
    } else {
        error!("error processing state");
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "error unwrapping status",
        ))
        .into());
    }
    info!("waiting messages to complite write to shared memory");
    //------------------------------------------------------
    read_shmem(MESSAGES_NUMBER, right_messages, opt);
    //------------------------------------------------------
    Ok(())
}

struct BaseRequest {}
enum Cmd1 {
    Establish,
    Sending(String),
}
impl BaseRequest {
    fn construct(pattern: Cmd1) -> tonic::Request<ClientRequest> {
        match pattern {
            Cmd1::Establish => tonic::Request::new(ClientRequest {
                command: ClientCommand::Connect.into(),
                timestamp: Some(std::time::SystemTime::now().into()),
                payload: Some("stream_storage".to_string()),
                serial: 0,
            }),
            Cmd1::Sending(s) => tonic::Request::new(ClientRequest {
                command: ClientCommand::Sending.into(),
                timestamp: Some(std::time::SystemTime::now().into()),
                payload: Some(s),
                serial: 0,
            }),
        }
    }
}
