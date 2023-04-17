#![allow(unused_imports)]
//#######################################################
//#######################################################
//#######################################################
// const KEY_LEN: usize = 10;
const _VAL_LEN: usize = 10;
const EXTRA_PRINT: bool = true;
const TEST_MODE: u32 = 1; /* 0 - SHNOOR , 1- ECDSA , 2- RANDOM */
//#######################################################
//#######################################################
//#######################################################
//-------------------------------------------------------
//-------------------------------------------------------
use log::*;
use std::io::stdin;
mod client_shmem;
use client_shmem::shmem_impl::*;
mod native_verification_schoor;
use native_verification_schoor::implement::*;
mod native_verification_ecdsa;
use native_verification_ecdsa::implement::*;

use libshmem::datastructs::*;
use redis::{
    from_redis_value,
    streams::{StreamRangeReply, StreamReadOptions, StreamReadReply},
    AsyncCommands, Client,
};
use transport::transport_interface_client::TransportInterfaceClient;
use transport::{ClientCommand, ClientRequest, ServerResponse, StatusMsg};
mod wasm_processor_wasmtime;
use wasm_processor_wasmtime::implement::*;
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
use k256::{PublicKey, SecretKey};
use rand_core::{OsRng, RngCore};
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
    let mut right_messages: Vec<Answer> = Vec::new(); // stream_storage
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
        for _i in 0..MESSAGES_NUMBER {
            // sending to server
            let msg_type = if TEST_MODE == 1 {
                Message::Ecdsa
            } else if TEST_MODE == 0 {
                Message::Shnoor
            } else {
                println!("{}", cyan.apply_to("generating randomly encoded message"));
                (OsRng.next_u32() % 2).try_into().unwrap()
            };
            let (signed_msg, unique_key, msg, smsg_len, _) = if msg_type == Message::Shnoor {
                construct_message(Message::Shnoor)
            } else {
                construct_message(Message::Ecdsa)
            };
            //push answer
            right_messages.push(Answer::new(msg.clone(), smsg_len, msg_type)); // fix the right msg
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

pub enum Runner {
    Wasmtime,
    Wasm3,
    Native,
    Replace,
}
#[derive(Clone, Copy, Debug, PartialEq)]
enum Message {
    Shnoor,
    Ecdsa,
}

impl TryFrom<u32> for Message {
    //     type Error = ();
    type Error = std::io::Error;

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == Message::Shnoor as u32 => Ok(Message::Shnoor),
            x if x == Message::Ecdsa as u32 => Ok(Message::Ecdsa),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "cant make into",
            )),
        }
    }
}
pub struct Answer {
    msg: String,
    e_len: usize,
    mtype: Message,
}
impl Answer {
    fn new(msg: String, e_len: usize, mtype: Message) -> Self {
        Self { msg, e_len, mtype }
    }
}

fn construct_message(type_msg: Message) -> (String, Vec<u8>, String, usize, Message) {
    let cyan = Style::new().cyan();
    match type_msg {
        Message::Shnoor => {
            let rng = RNG::try_from(&Language::Roman).unwrap();
            let signing_key = SigningKey::random(&mut OsRng); // generate sign key
            let verifying_key_bytes: [u8; 32] = signing_key
                .verifying_key()
                .to_bytes()
                .as_slice()
                .try_into()
                .expect("wrong length"); // 32-bytes VERIFY KEY
            let unique_key: Vec<u8> = verifying_key_bytes.into(); // verify key to vec
            let msg = rng.generate_name();
            println!("generating message: {} [SHNOOR]", cyan.apply_to(&msg));
            let signatured_msg = signing_key.sign(msg.as_bytes()).to_bytes(); // sign msg
            let signed_msg = hex::encode(signatured_msg); // encode signed
            let smsg_len = signed_msg.len();
            let msg_type = Message::Shnoor;
            (signed_msg, unique_key, msg, smsg_len, msg_type)
        }
        Message::Ecdsa => {
            let rng = RNG::try_from(&Language::Roman).unwrap();
            let signing_key = k256::ecdsa::SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
            let unique_key = signing_key.verifying_key();
            let unique_key = unique_key.to_sec1_bytes();
            let msg = rng.generate_name();
            println!("generating message: {} [ECDSA]", cyan.apply_to(&msg),);
            let msg1 = msg.as_bytes();
            let signatured_msg =
                Signer::<ecdsa::Signature<k256::Secp256k1>>::sign(&signing_key, msg1);
            let signed_msg = hex::encode(signatured_msg.to_der()); // encode signed
            println!(
                "smsg len: {}, encoded {}",
                signatured_msg.to_bytes().len(),
                signed_msg.len()
            );
            let smsg_len = signed_msg.len();
            let msg_type = Message::Ecdsa;
            (signed_msg, unique_key.into(), msg, smsg_len, msg_type)
        }
    }
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
