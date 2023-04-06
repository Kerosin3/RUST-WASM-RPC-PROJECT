#![allow(unused_imports)]
use std::io::stdin;
extern crate hex_slice;
//use anyhow::{Ok, Result};
// use std::io::{Error, ErrorKind};
use transport::transport_interface_client::TransportInterfaceClient;
use transport::{ClientCommand, ClientRequest, ServerResponse, StatusMsg};
pub mod transport {
    tonic::include_proto!("transport_interface");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TransportInterfaceClient::connect("http://[::1]:8080")
        .await
        .unwrap();
    let response = client
        .establish_connection(BaseRequest::construct(Cmd1::Establish))
        .await
        .unwrap();
    let init = response.into_inner();

    if let Some(t) = StatusMsg::from_i32(init.msg_status) {
        if t != StatusMsg::Proceed {
            //             return Err(std::io::Error:: .into());
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "fron initial connect",
            ))
            .into());
        }
    } else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "error unwrapping status",
        ))
        .into());
    }
    println!("finishing!");
    //------------------------------------------------------
    loop {
        take_input();
    }
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
                payload: None,
                serial: 0,
            }),
            Cmd1::Sending(s) => tonic::Request::new(ClientRequest {
                command: ClientCommand::Connect.into(),
                timestamp: Some(std::time::SystemTime::now().into()),
                payload: Some(s),
                serial: 0,
            }),
        }
    }
}
