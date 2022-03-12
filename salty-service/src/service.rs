use crate::cmd_processor::CmdProcessor;
use crate::config::Config;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::{self, sync::mpsc, task};

pub use tonic::{Request, Response, Status};
pub mod salty {
    tonic::include_proto!("salty");
}
pub use salty::CommandResponse;
pub use salty::{
    vault_server::{Vault, VaultServer},
    CommandRequest,
};
use salty_utils::logs;

#[cfg(test)]
mod tests_vault_service {
    use super::VaultService;
    use crate::config::{Config, Testing};
    use std::time::{Duration, Instant};
    use tokio::{self, sync::mpsc};

    async fn set_up(shutdown_timeout: u64) -> (VaultService, mpsc::Receiver<i32>) {
        let (tx, rx) = mpsc::channel(32);
        let mut config = Config::default();
        config.shutdown_timeout = shutdown_timeout;
        (VaultService::new(tx.clone(), config), rx)
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

pub struct VaultService {
    sender: mpsc::Sender<i32>,
    timer_handler: Arc<Mutex<tokio::task::JoinHandle<()>>>,
    config: Config,
    processor: CmdProcessor,
}

impl Drop for VaultService {
    fn drop(&mut self) {
        logs::info!("VaultService stopped!");
    }
}

impl VaultService {
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
            logs::info!("Stoping VaultServer!");
        })
    }
    fn reset_shutdown(&self) {
        self.timer_handler.lock().unwrap().abort();
        *self.timer_handler.lock().unwrap() = VaultService::schedule_shutdown(
            self.sender.clone(),
            self.config.shutdown_timeout * 1000,
        );
    }
    pub fn new(sender: mpsc::Sender<i32>, config: Config) -> Self {
        let timer = Arc::new(Mutex::new(VaultService::schedule_shutdown(
            sender.clone(),
            config.shutdown_timeout * 1000,
        )));
        let vault = VaultService {
            processor: CmdProcessor::default(&config),
            config: config,
            sender: sender,
            timer_handler: timer,
        };
        vault
    }
}

#[tonic::async_trait]
impl Vault for VaultService {
    async fn process_cmd(
        &self,
        request: Request<CommandRequest>,
    ) -> Result<Response<CommandResponse>, Status> {
        logs::info!("Got a request from {:?}", request.remote_addr());
        self.reset_shutdown();
        let reply = self.processor.handle(request.get_ref());
        Ok(Response::new(reply))
    }
}
