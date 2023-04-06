#![allow(unused)] // silence unused warnings while exploring (to comment out)
use libmoses::runme;
use redis::{
    from_redis_value,
    streams::{StreamRangeReply, StreamReadOptions, StreamReadReply},
    AsyncCommands, Client,
};
use std::{error::Error, time::Duration};
use tokio::time::sleep;
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    runme(5, 5);
    let client = Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_tokio_connection().await?;
    con.set("my_key", "Hello world!").await?;
    let result: String = con.get("my_key").await?;
    println!("->> my_key: {}\n", result);
    Ok(())
}
