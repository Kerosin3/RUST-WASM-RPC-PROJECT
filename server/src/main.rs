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
    /*
    let redis_connection = Client::open("redis://127.0.0.1").unwrap();
    let mut con = redis_connection.get_tokio_connection().await.unwrap();
    let len: i32 = con.xlen("my_stream").await.unwrap();
    println!("->> my_stream len {}\n", len);
    let result: Option<StreamRangeReply> = con
        .xrevrange_count("stream_storage", "+", "-", 50)
        .await
        .unwrap();
    if let Some(reply) = result {
        for stream_id in reply.ids {
            println!("deleting {} {}", "my_stream", stream_id.id);
            //             let result: RedisResult<i32> = con.xdel("my_stream", &[stream_id.id.clone()]).await?;
            println!("->> xrevrange stream entity: {}  ", stream_id.id);
            for (name, value) in stream_id.map.iter() {
                println!(
                    "  ->> {}: {}",
                    name,
                    from_redis_value::<String>(value).unwrap()
                );
            }
            println!();

            let result: RedisResult<i32> =
                con.xdel("stream_storage", &[stream_id.id.clone()]).await;
        }
    } */

    let address = "[::1]:8080".parse().unwrap();
    let server_main_service = RpcServiceServer::default();
    //     let file_appender = tracing_appender::rolling::hourly(".", "test.log");
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
