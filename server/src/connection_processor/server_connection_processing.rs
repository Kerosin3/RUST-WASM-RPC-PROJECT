pub mod Implement {
    use crate::connection_processor::input::InputProcessor::parse_input;
    use mockall::predicate::*;
    use mockall::*;
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

    #[derive(Debug, Default)]
    pub struct RpcServiceServer {}
    // mocking
    #[tonic::async_trait]
    //     #[mockall::automock]
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
                        Ok(tonic::Response::new(transport::ServerResponse {
                            server_answer: { None },
                            msg_status: StatusMsg::Proceed.into(),
                            timestamp: Some(std::time::SystemTime::now().into()),
                        }))
                    }
                    ClientCommand::Sending => Ok(tonic::Response::new(transport::ServerResponse {
                        server_answer: { name },
                        msg_status: StatusMsg::Ok.into(),
                        timestamp: Some(std::time::SystemTime::now().into()),
                    })),
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
