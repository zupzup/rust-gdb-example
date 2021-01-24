use std::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Accepting TCP on port 8080");

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move { process(socket).await });
    }
}

async fn process(mut socket: TcpStream) {
    socket
        .write_all(b"Hello")
        .await
        .expect("can write to socket");
}
