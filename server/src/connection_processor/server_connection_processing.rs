pub mod Implement {
    use crate::connection_processor::input::InputProcessor::parse_input;
    use mockall::predicate::*;
    use mockall::*;
    use rnglib::{Language, RNG};
    use transport::transport_interface_server::{TransportInterface, TransportInterfaceServer};
    use transport::{ClientCommand, ClientRequest, Connection, ServerResponse};
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

    #[derive(Debug, Default)]
    pub struct RpcServiceServer {}
    // mocking
    #[tonic::async_trait]
    #[mockall::automock]
    impl TransportInterface for RpcServiceServer {
        async fn establish_connection(
            &self,
            request: Request<ClientRequest>,
        ) -> Result<Response<ServerResponse>, Status> {
            let recv_from_client = request.into_inner();
            if let Some(t) = Connection::from_i32(recv_from_client.type_c) {
                match t {
                    Connection::Client => {
                        tracing::info!("accepting new client");
                        // parsing command from client
                        if let Some(com) = ClientCommand::from_i32(recv_from_client.command) {
                            match com {
                                ClientCommand::InitName => {
                                    let rng = RNG::try_from(&Language::Elven).unwrap();
                                    let name = rng.generate_name();
                                    tracing::info!("assigning name:{}", name);
                                    Ok(tonic::Response::new(transport::ServerResponse {
                                        server_answer: { name },
                                    }))
                                }
                                _ => Err(Status::invalid_argument("not acceptable here")),
                            }
                        } else {
                            Err(Status::unknown("unknown agent type"))
                        }
                    }
                    Connection::Agent => {
                        tracing::info!("connection agent!");
                        Ok(tonic::Response::new(transport::ServerResponse {
                            server_answer: { "connecting agent".to_string() },
                        }))
                    }
                }
            } else {
                tracing::info!("unknown type deserealization");
                Err(Status::out_of_range("no such agent type"))
            }
        }
        async fn assign_arena(
            &self,
            request: Request<ClientRequest>,
        ) -> Result<Response<ServerResponse>, Status> {
            let recv_from_client = request.into_inner();
            if let Some(r) = Connection::from_i32(recv_from_client.type_c) {
                match r {
                    Connection::Client => {
                        if let Some(command) = ClientCommand::from_i32(recv_from_client.command) {
                            if command == ClientCommand::AssignBattle {
                                let rng = RNG::try_from(&Language::Fantasy).unwrap();
                                let name = rng.generate_name();
                                tracing::info!("assigning battle name:{}", name);
                                Ok(tonic::Response::new(transport::ServerResponse {
                                    server_answer: { name },
                                }))
                            } else {
                                Err(Status::aborted("Not acceptable here"))
                            }
                        } else {
                            todo!()
                        }
                    }
                    Connection::Agent => Err(Status::out_of_range("no such agent type")),
                }
            } else {
                tracing::info!("unknown type deserealization");
                Err(Status::out_of_range("no such agent type"))
            }
        }

        async fn client_command(
            &self,
            request: Request<ClientRequest>,
        ) -> Result<Response<ServerResponse>, Status> {
            let recv_from_client = request.into_inner();
            if let Some(r) = Connection::from_i32(recv_from_client.type_c) {
                match r {
                    Connection::Client => {
                        if let Some(command) = ClientCommand::from_i32(recv_from_client.command) {
                            if command == ClientCommand::Control {
                                tracing::info!("analysing command :{:?}", recv_from_client.payload);
                                if recv_from_client.payload.is_none() {
                                    return Err(Status::aborted("Wrong payload"));
                                } else {
                                    Ok(tonic::Response::new(transport::ServerResponse {
                                        server_answer: { "Ok".to_string() },
                                    }))
                                }
                            } else {
                                Err(Status::aborted("Not acceptable here"))
                            }
                        } else {
                            todo!()
                        }
                    }
                    Connection::Agent => Err(Status::out_of_range("no such agent type")),
                }
            } else {
                tracing::info!("unknown type deserealization");
                Err(Status::out_of_range("no such agent type"))
            }
        }
    }
}
