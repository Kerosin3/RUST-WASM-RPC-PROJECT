use tokio::io::BufStream;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{sleep, Duration};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[tokio::main(flavor = "multi_thread", worker_threads = 3)]
async fn main() {
    if let Ok(tcp_listener) = TcpListener::bind("127.0.0.1:12345").await {
        while let Ok((tcp_stream, _socket_addr)) = tcp_listener.accept().await {
            tokio::spawn(async move { process_connection(tcp_stream) })
                .await
                .unwrap();
        }
    }
}

fn process_connection(tcp_stream: TcpStream) {
    let codec = LengthDelimitedCodec::builder()
        .length_field_offset(0) // default value
        .length_field_type::<u16>()
        .length_adjustment(0) // default value
        .new_codec();
    let stream_buf = BufStream::new(tcp_stream);
    let mut framed_stream = Framed::new(stream_buf, codec);

    //     let frame = Bytes::from("hello from server, what do you want?");
    //     tracing::info!("sent hello to client");
    //     framed_stream.send(frame).await.unwrap();
}
