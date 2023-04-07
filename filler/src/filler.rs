#![allow(unused_imports)]
use std::io::stdin;
extern crate hex_slice;
//use anyhow::{Ok, Result};
// use std::io::{Error, ErrorKind};
use random_string::generate;
use rnglib::{Language, RNG};
use transport::transport_interface_client::TransportInterfaceClient;
use transport::{ClientCommand, ClientRequest, ServerResponse, StatusMsg};
pub mod transport {
    tonic::include_proto!("transport_interface");
}
use redis::{
    from_redis_value,
    streams::{StreamRangeReply, StreamReadOptions, StreamReadReply},
    AsyncCommands, Client,
};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
            // error
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "fron initial connect",
            ))
            .into());
        }
        //-----------------------------
        // adding message
        let rng = RNG::try_from(&Language::Roman).unwrap();
        for _i in 0..=9 {
            // sending to server
            let unique_key = generate(9, charset);
            let msg = rng.generate_name();
            con.xadd("stream_storage", "*", &[(unique_key, msg.clone())])
                .await?;
        }
        // send answer OK
        let _response = client
            .establish_connection(BaseRequest::construct(Cmd1::Sending(
                "stream_storage".to_string(),
            )))
            .await
            .unwrap();
    } else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "error unwrapping status",
        ))
        .into());
    }
    println!("finishing!");
    //------------------------------------------------------
    Ok(())
    /*
    loop {
        let inpt = take_input();
        con.set("my_key", "my val").await?;
        //         let result: String = con.get("my_key").await?;
    }*/
}
fn take_input() -> String {
    println!("\n-----type a command------");
    let mut some_input = String::new();
    stdin().read_line(&mut some_input).unwrap();
    let some_input = some_input.trim();
    let out = format!("#{}", some_input);
    println!("your command: {out}");
    out
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
