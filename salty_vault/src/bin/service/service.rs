use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::{self, sync::mpsc, task};
pub use tonic::{Request, Response, Status};
pub mod salty {
    tonic::include_proto!("salty");
}
use salty::CommandResponse;

pub use salty::{
    vault_server::{Vault, VaultServer},
    CommandRequest,
};

pub struct MyVault {
    sender: mpsc::Sender<i32>,
    timer_handler: Arc<Mutex<tokio::task::JoinHandle<()>>>,
}

impl MyVault {
    fn schedule_shutdown(sender: mpsc::Sender<i32>) -> tokio::task::JoinHandle<()> {
        task::spawn(async move {
            let init_time = Instant::now();
            loop {
                tokio::time::sleep(Duration::from_millis(500)).await;
                if init_time.elapsed() >= Duration::from_secs(30u64) {
                    break;
                }
            }
            sender.send(1).await;
            sender.closed();
            println!("Stoping VaultServer!");
        })
    }
    fn reset_shutdown(&self) {
        drop(self.timer_handler.lock());
        *self.timer_handler.lock().unwrap() = MyVault::schedule_shutdown(self.sender.clone());
    }
    pub fn new(sender: mpsc::Sender<i32>) -> Self {
        let timer = Arc::new(Mutex::new(MyVault::schedule_shutdown(sender.clone())));
        let vault = MyVault {
            sender: sender,
            timer_handler: timer,
        };
        vault
    }
}

#[tonic::async_trait]
impl Vault for MyVault {
    async fn send(
        &self,
        request: Request<CommandRequest>,
    ) -> Result<Response<CommandResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        self.reset_shutdown();
        let reply = salty::CommandResponse {
            message: format!("response sent!"),
        };
        Ok(Response::new(reply))
    }
}

impl Drop for MyVault {
    fn drop(&mut self) {
        println!("MyVault stopped!");
    }
}
