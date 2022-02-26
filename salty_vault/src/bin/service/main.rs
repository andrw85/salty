mod service;
use clap::Parser;
use service::VaultServer;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use tokio::{self, sync::mpsc, task};
use tonic::transport::Server;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Flags {
    /// Sets the port number to listen on.
    #[clap(long, default_value_t = 50051)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let flags = Flags::parse();
    let addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), flags.port);

    let (tx, mut rx) = mpsc::channel(32);
    let vault = service::MyVault::new(tx.clone());

    task::spawn(async move {
        println!("VaultServer listening on {}", addr);
        Server::builder()
            .add_service(VaultServer::new(vault))
            // .serve_with_shutdown(addr, rx.recv().map(drop))
            .serve(addr)
            .await
            .unwrap();
    });

    rx.recv().await;
    println!("Terminating daemon!");
    Ok(())
}
