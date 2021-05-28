use tokio;

use clap::{AppSettings, Clap};
use tokio::io::AsyncReadExt;
mod util;
use log::{error, info, warn};
use pretty_env_logger;
use std::env;
use std::net::ToSocketAddrs;
use std::time::Duration;
use zeroconf::prelude::*;
use zeroconf::MdnsService;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "0.0.0.0:7890")]
    listen: String,
}

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
async fn main() {
    env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    let opts: Opts = Opts::parse();

    //Monstrosity
    let listenaddr = opts.listen.to_socket_addrs().unwrap().next().unwrap();
    info!("OPC Server listening on: {}", listenaddr);

    //SPI setup

    let server = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(&opts.listen).await.unwrap();

        loop {
            match listener.accept().await {
                Ok((socket, addr)) => process_socket(socket, addr).await,
                Err(v) => {
                    error!("Socket: {}", v);
                }
            }
        }
    });

    let advertiser = tokio::spawn(async move {
        let mut service = MdnsService::new("_opc._tcp", listenaddr.port());
        let event_loop = service.register().unwrap();

        loop {
            event_loop.poll(Duration::from_secs(5)).unwrap();
        }
    });

    tokio::join!(server, advertiser);
}
