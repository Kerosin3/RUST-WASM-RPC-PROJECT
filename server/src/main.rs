#![allow(unused_imports)]
#![feature(vec_into_raw_parts)]

use prost_types::Timestamp;
//use std::time::SystemTime;
use connection_processor::server_connection_processing::Implement;
use libshmem::datastructs::*;
use libshmem::datastructs::*;
use shared_memory::*;
use std::path::Path;
use tonic::{transport::Server, Request, Response, Status};
use tracing::Level;
use tracing_subscriber::fmt;
use transport::transport_interface_server::{TransportInterface, TransportInterfaceServer};
mod connection_processor;
use connection_processor::server_connection_processing::Implement::*;
use redis::streams::*;
use redis::{
    from_redis_value,
    streams::{StreamRangeReply, StreamReadOptions, StreamReadReply},
    AsyncCommands, Client,
};
use redis::{Commands, Connection, RedisResult, ToRedisArgs};
//main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = fmt()
        .compact()
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let root_path = project_root::get_project_root().unwrap();
    let shmem_flink = Path::new(&root_path).join(MEMFILE);
    let _ = std::fs::remove_file(&shmem_flink);
    let memsize: usize = MEMSIZE;
    let _shmem = match ShmemConf::new().size(memsize).flink(shmem_flink).create() {
        Ok(mem) => mem,
        Err(ShmemError::LinkExists) => {
            eprintln!("shared memory exists!");
            panic!();
        }
        Err(e) => {
            eprintln!("Unable to create or open shmem flink  {e}");
            panic!();
        }
    };

    tracing::info!(
        "working with shared memory file {} with size {}",
        MEMFILE,
        MEMSIZE
    );
    let address = "[::1]:8080".parse().unwrap();
    let server_main_service = RpcServiceServer::default();
    tracing::info!("start main server loop on 8080 port");
    Server::builder()
        .add_service(TransportInterfaceServer::new(server_main_service))
        .serve(address)
        .await?;

    Ok(())
}
