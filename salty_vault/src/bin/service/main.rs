mod service;
// use futures::channel::mpsc;
use futures::future::FutureExt;
use service::VaultServer;
use std::time::{Duration, Instant};
use tokio::{self, sync::mpsc, task};

use tonic::transport::Server;
#[derive(Debug, Clone)]
struct Mio {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    let (tx, mut rx) = mpsc::channel(32);
    let vault = service::MyVault::new(tx.clone());

    let server = task::spawn(async move {
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
