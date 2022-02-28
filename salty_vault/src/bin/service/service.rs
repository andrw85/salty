use crate::config::Config;
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

#[cfg(test)]
mod tests_vault_server {
    use super::{Config, MyVault};
    use std::time::{Duration, Instant};
    use tokio::{self, sync::mpsc};

    async fn set_up(shutdown_timeout: u64) -> (MyVault, mpsc::Receiver<i32>) {
        let (tx, rx) = mpsc::channel(32);
        let mut config = Config::default();
        config.shutdown_timeout = shutdown_timeout;
        (MyVault::new(tx.clone(), config), rx)
    }

    async fn tear_down(mut rx: mpsc::Receiver<i32>) {
        let result = rx.recv().await;
        assert_ne!(result, None);
    }

    #[tokio::test]
    async fn test_shutdown_when_timeout_is_zero() {
        let shutdown_timeout = 0u64;
        let init_time = Instant::now();
        let (_, rx) = set_up(shutdown_timeout).await;
        tear_down(rx).await;
        // took no more of 3ms running the test
        assert!(init_time.elapsed() <= Duration::from_millis(3u64));
    }

    #[tokio::test]
    async fn test_shutdown_when_timeout_bigger_than_zero() {
        let shutdown_timeout = 3u64;
        let init_time = Instant::now();
        let (_, rx) = set_up(shutdown_timeout).await;
        tear_down(rx).await;
        // make sure shutdown timeout was working
        // check if it took at leat 3ms running the test before shutting down.
        assert!(init_time.elapsed() >= Duration::from_millis(shutdown_timeout));
    }
}

pub struct MyVault {
    sender: mpsc::Sender<i32>,
    timer_handler: Arc<Mutex<tokio::task::JoinHandle<()>>>,
    config: Config,
}

impl Drop for MyVault {
    fn drop(&mut self) {
        println!("MyVault stopped!");
    }
}

impl MyVault {
    fn schedule_shutdown(
        sender: mpsc::Sender<i32>,
        shutdown_timeout: u64,
    ) -> tokio::task::JoinHandle<()> {
        task::spawn(async move {
            let init_time = Instant::now();
            loop {
                tokio::time::sleep(Duration::from_millis(shutdown_timeout)).await;
                if init_time.elapsed() >= Duration::from_millis(shutdown_timeout) {
                    break;
                }
            }
            sender.send(1).await.unwrap();
            sender.closed().await;
            println!("Stoping VaultServer!");
        })
    }
    fn reset_shutdown(&self) {
        drop(self.timer_handler.lock());
        *self.timer_handler.lock().unwrap() =
            MyVault::schedule_shutdown(self.sender.clone(), self.config.shutdown_timeout);
    }
    pub fn new(sender: mpsc::Sender<i32>, config: Config) -> Self {
        let timer = Arc::new(Mutex::new(MyVault::schedule_shutdown(
            sender.clone(),
            config.shutdown_timeout,
        )));
        let vault = MyVault {
            sender: sender,
            timer_handler: timer,
            config: config,
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
