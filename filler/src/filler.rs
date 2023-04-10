#![allow(unused_imports)]
//#######################################################
//#######################################################
//#######################################################
const KEY_LEN: usize = 10;
const _VAL_LEN: usize = 10;
//#######################################################
//#######################################################
//#######################################################
//-------------------------------------------------------
//-------------------------------------------------------
use std::io::stdin;
extern crate hex_slice;
use log::*;
mod client_shmem;
use client_shmem::shmem_impl::*;
use libshmem::datastructs::*;
use random_string::generate;
use redis::{
    from_redis_value,
    streams::{StreamRangeReply, StreamReadOptions, StreamReadReply},
    AsyncCommands, Client,
};
use rnglib::{Language, RNG};
use transport::transport_interface_client::TransportInterfaceClient;
use transport::{ClientCommand, ClientRequest, ServerResponse, StatusMsg};
mod wasm_processor;
use wasm_processor::implement::*;
pub mod transport {
    tonic::include_proto!("transport_interface");
}
use base64::{engine::general_purpose, Engine as _};
use crossbeam_channel::unbounded;
use k256::schnorr::{
    signature::{Signer, Verifier},
    Signature, SignatureBytes, SigningKey, VerifyingKey,
};
use rand_core::OsRng;
//-------------------------------------------------------
//-------------------------------------------------------
//-------------------------------------------------------
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("starting client app");
    let charset = "abcdefg123456789";
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
    if let Some(t) = StatusMsg::from_i32(init.msg_status) {
        if t != StatusMsg::Proceed {
            error!("cannot connect to server");
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "fron initial connect",
            ))
            .into());
        }
        //-----------------------------
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
            //let msg = String::from("hehe");
            let signatured_msg = signing_key.sign(msg.as_bytes()).to_bytes(); // sign msg
            let signed_msg = hex::encode(&signatured_msg); // encode signed
                                                           //-----
                                                           //  println!("size is {}", signed_msg.len());
            let restored_signed_message = hex::decode(&signed_msg).unwrap(); // decode S msg back
            let x = Signature::try_from(&restored_signed_message[..]).unwrap();

            let ver_key = VerifyingKey::from_bytes(&verifying_key_bytes).unwrap(); // restore v key
            assert!(ver_key.verify(msg.as_bytes(), &x).is_ok());
            //let signed_msg = general_purpose::STANDARD_NO_PAD.encode(&signatured_msg); // encode b64
            println!(
                "---[{}]---key={},S_MESSAGE={:?}",
                _i,
                hex::encode(&unique_key), //ver key
                signed_msg                // msg signed
            );
            /*println!(
                "ENCODED: MSG {} V_KEY:{}",
                hex::encode(&signed_msg),
                hex::encode(&unique_key)
            );*/

            con.xadd("stream_storage", "*", &[(signed_msg.clone(), unique_key)])
                .await?;
        }
        // send answers OK
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
    //     std::thread::sleep(std::time::Duration::from_secs(10));
    info!("waiting messages to complite write to shared memory");
    //------------------------------------------------------
    read_shmem(MESSAGES_NUMBER);
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
