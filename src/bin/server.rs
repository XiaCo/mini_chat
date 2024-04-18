use std::fmt::Write;

use bytes::{Buf, BytesMut};
use mini_chat::{server};
use clap::Parser;
use tokio::{net::TcpListener, signal};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ServerConf {
    #[arg(long)]
    host: String,

    #[arg(short, long, default_value_t = 80)]
    port: u16,
}

#[tokio::main]
pub async fn main() {
    // let sc = ServerConf::parse();
    
    // Bind a TCP listener
    // let listener = TcpListener::bind(&format!("{}:{}", sc.host, sc.port)).await?;

    // server::run(listener, signal::ctrl_c()).await;
}


