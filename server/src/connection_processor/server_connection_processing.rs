pub mod Implement {
    use crate::connection_processor::input::InputProcessor::parse_input;
    use anyhow::Result;
    use mockall::predicate::*;
    use mockall::*;
    use redis::streams::*;
    use redis::{
        from_redis_value,
        streams::{StreamRangeReply, StreamReadOptions, StreamReadReply},
        AsyncCommands, Client,
    };
    use redis::{Commands, Connection, RedisResult, ToRedisArgs};
    use rnglib::{Language, RNG};
    use transport::transport_interface_server::{TransportInterface, TransportInterfaceServer};
    use transport::{ClientCommand, ClientRequest, ServerResponse, StatusMsg};
    pub mod transport {
        // import proto
        tonic::include_proto!("transport_interface");
    }
    use tonic::{transport::Server, Request, Response, Status};
    //  use crate::RpcServiceServer;
    /*    pub fn printsome() {
        println!("aaaaaaaaaa");
        let mut mock_server = Box::new(MockRpcServiceServer::new());
    }*/
    pub struct SharedData {
        continue_background_tasks: bool,
        data: [u8: 1048576] // 1 MB
    }
    #[derive(Debug, Default)]
    pub struct RpcServiceServer {}
    // mocking
    #[tonic::async_trait]
    impl TransportInterface for RpcServiceServer {
        async fn establish_connection(
            &self,
            request: Request<ClientRequest>,
        ) -> Result<Response<ServerResponse>, Status> {
            let recv_from_client = request.into_inner();
            let name = recv_from_client.payload;
            if let Some(t) = ClientCommand::from_i32(recv_from_client.command) {
                match t {
                    ClientCommand::Connect => {
                        tracing::info!("accepted connection!");
                        if let Some(stream_name) = name {
                            Ok(tonic::Response::new(transport::ServerResponse {
                                server_answer: { None },
                                msg_status: StatusMsg::Proceed.into(),
                                timestamp: Some(std::time::SystemTime::now().into()),
                            }))
                        } else {
                            tracing::info!("error getting stream name");
                            Ok(tonic::Response::new(transport::ServerResponse {
                                server_answer: { None },
                                msg_status: StatusMsg::Error.into(),
                                timestamp: Some(std::time::SystemTime::now().into()),
                            }))
                        }
                    }
                    ClientCommand::Sending => {
                        tracing::info!("processing message pack");
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
                        }
                        Ok(tonic::Response::new(transport::ServerResponse {
                            server_answer: { name },
                            msg_status: StatusMsg::Ok.into(),
                            timestamp: Some(std::time::SystemTime::now().into()),
                        }))
                    }
                }
            } else {
                // error
                Ok(tonic::Response::new(transport::ServerResponse {
                    server_answer: { None },
                    msg_status: StatusMsg::Error.into(),
                    timestamp: Some(std::time::SystemTime::now().into()),
                }))
            }
        }
    }
}
