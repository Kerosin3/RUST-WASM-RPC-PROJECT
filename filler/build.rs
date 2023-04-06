fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional") // for older systems
        .build_client(true)
        .build_server(true)
        .build_transport(true)
        .compile(&["server_client.proto"], &["../proto"])?;
    //         compile_protos("../proto/server_client.proto")?;
    Ok(())
}
