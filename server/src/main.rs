#![allow(unused_imports)]
use prost_types::Timestamp;
//use std::time::SystemTime;
use blake2::{Blake2b512, Blake2s256, Digest};
use tonic::{transport::Server, Request, Response, Status};
use tracing::Level;
use tracing_subscriber::fmt;
//
use connection_processor::server_connection_processing::Implement;
use transport::transport_interface_server::{TransportInterface, TransportInterfaceServer};
mod connection_processor;
use connection_processor::server_connection_processing::Implement::*;
//main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:8080".parse().unwrap();
    let server_main_service = RpcServiceServer::default();

    let subscriber = fmt()
        .compact()
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    tracing::info!("start main server loop");
    Server::builder()
        .add_service(TransportInterfaceServer::new(server_main_service))
        .serve(address)
        .await?;
    Ok(())
}
