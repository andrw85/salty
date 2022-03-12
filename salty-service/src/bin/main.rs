use salty_service::{Config, Parser, VaultServer, VaultService};
use salty_utils::logs;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use tokio::{self, sync::mpsc, task};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();
    let addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), config.port);

    let (tx, mut rx) = mpsc::channel(32);
    let vault = VaultService::new(tx.clone(), config);

    task::spawn(async move {
        logs::info!("VaultServer listening on {}", addr);
        Server::builder()
            .add_service(VaultServer::new(vault))
            // .serve_with_shutdown(addr, rx.recv().map(drop))
            .serve(addr)
            .await
            .unwrap();
    });

    rx.recv().await;
    logs::info!("Terminating daemon!");
    Ok(())
}
