use std::env;

use super::options;
use fork::{daemon, Fork};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::net::UnixStream;
use std::process;
// use tokio::io;
use crate::vault_cli::VaultDaemon;
use std::fs;
use std::path::Path;

const SOCKET_NAME: &str = "salty.sock";

pub struct CliClient {
    // vault: VaultDaemon,
    socket: String,
}

impl CliClient {
    fn new() -> CliClient {
        let default_dir = dirs::home_dir()
            .expect("No home directory found in your system!")
            .join(".salty/");

        let path_socket = default_dir
            .join(SOCKET_NAME)
            .to_str()
            .expect("invalid path to socket!")
            .to_owned();

        let pid_file = default_dir.join("pid");

        if !Path::new(&default_dir).exists() {
            println!("Creating salty directory");
            fs::create_dir(default_dir).ok();
        }

        if !pid_file.as_path().exists() {
            ///https://docs.rs/fork/0.1.18/fork/fn.daemon.html
            if let Ok(Fork::Child) = daemon(false, true) {
                println!(
                    "Created vault process! {}",
                    pid_file.as_path().to_str().unwrap()
                );
                let mut file = File::create(pid_file.as_path()).expect("Failed creating Pid file");
                file.write_all(std::process::id().to_string().as_bytes());
                file.flush();
                VaultDaemon::run();
            }
            std::process::exit(0); // exit cli process
        }

        CliClient {
            socket: path_socket,
        }
    }
    /// https://gist.github.com/tesaguri/b27d0d35d1a45465ddc9cb32a3ebe9ae
    /// https://docs.rs/tokio/1.13.0/tokio/net/struct.UnixStream.html
    fn send_command(self, opt: options::Opt) -> Result<(), Box<dyn Error>> {
        println!("sending command to backend!");
        let mut stream = UnixStream::connect(&self.socket).expect("Failed connecting to socket");
        stream.write_all(&serde_json::to_string(&opt).unwrap().into_bytes())?;

        // loop {
        // get response from backend
        let mut buffer = String::new();
        stream.read_to_string(&mut buffer)?;
        // }

        Ok(())
    }
    pub fn run() {
        if env::args_os().len() == 1 {
            // when no CLI arguments
            // println!("Default vault in ~/.salty/");
            std::process::exit(0); // exit cli process
            return;
        }

        let vault_cli = Self::new();
        let opt = options::options();
        vault_cli.send_command(opt);
    }
}
