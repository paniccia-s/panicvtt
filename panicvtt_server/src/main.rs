use std::{net::Ipv4Addr, thread, time::Duration};

use panicvtt_engine;
use panicvtt_net::panicnet::PanicNetServer;

mod version;

pub fn version() -> &'static str {
    version::VERSION
}

#[tokio::main]
async fn main() {
    println!("PanicVTT Server version {}: Engine version {}", version(), panicvtt_engine::version());

    let panicnet = PanicNetServer::new();
    let _ = panicnet.start(Ipv4Addr::LOCALHOST, 21918).await.unwrap();

    tokio::signal::ctrl_c().await.unwrap_or_else(|_| {
        eprintln!("Failed to register ctrl-c handler!");
        std::process::exit(1);
    });
    
    panicnet.test();
    panicnet.close();
}
