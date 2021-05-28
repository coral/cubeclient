use tokio;

use std::error::Error;
use std::io;
use tokio::{io::AsyncRead, io::AsyncReadExt, net::TcpListener, net::TcpStream, task};
mod util;

async fn process_socket(mut stream: tokio::net::TcpStream, addr: std::net::SocketAddr) {
    loop {
        let channel = stream.read_u8().await.unwrap();
        let command = stream.read_u8().await.unwrap();
        let size = stream.read_u16().await.unwrap();
        let mut data = vec![0u8; size as usize];
        stream.read_exact(&mut data).await;

        let m: Vec<util::Color> = data
            .chunks_exact(3)
            .map(|v| util::Color {
                r: v[0],
                g: v[1],
                b: v[2],
            })
            .collect();

        dbg!(m);
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7890").await?;

    loop {
        let (socket, addr) = listener.accept().await?;
        tokio::spawn(async move { process_socket(socket, addr).await });
    }
}
