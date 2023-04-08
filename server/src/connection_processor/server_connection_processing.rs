pub mod Implement {
    //----------------------------------------------------------------//

    //----------------------------------------------------------------//
    //----------------------------------------------------------------//
    use crate::connection_processor::shmem_server::memoryprocessor::*;
    use anyhow::Result;
    use libshmem::datastructs::MESSAGES_NUMBER;
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
        tonic::include_proto!("transport_interface");
    }
    use tonic::{transport::Server, Request, Response, Status};
    //----------------------------------------------------------------//
    //----------------------------------------------------------------//
    //----------------------------------------------------------------//
    #[derive(Debug, Default)]
    pub struct RpcServiceServer {}
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
                        if let Some(_stream_name) = name {
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
                        tracing::info!("connected to redis!");
                        let mut con = redis_connection.get_tokio_connection().await.unwrap();
                        let _len: i32 = con.xlen("my_stream").await.unwrap();
                        let result: Option<StreamRangeReply> = con
                            .xrevrange_count("stream_storage", "+", "-", MESSAGES_NUMBER)
                            .await
                            .unwrap();
                        if let Some(reply) = result {
                            let mut j: u32 = 0;
                            for stream_id in reply.ids {
                                //        println!("deleting {} {}", "my_stream", stream_id.id);
                                //      println!("->> xrevrange stream entity: {}  ", stream_id.id);
                                for (name, value) in stream_id.map.iter() {
                                    let val = from_redis_value::<String>(value).unwrap();

                                    tracing::info!("deleting: [serial:{}  {}: {}]", j, name, val);
                                    fill_sh_memory(name.to_string(), val, j);
                                    j += 1;
                                }
                                println!();
                                let _result: RedisResult<i32> =
                                    con.xdel("stream_storage", &[stream_id.id.clone()]).await;
                            }
                            write_complite()
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
