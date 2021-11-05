use super::generator::*;
use super::options::{AddOpt, Opt};

use dirs;
use fork::{daemon, Fork};
use futures::{Future, FutureExt, TryFutureExt};
use question::{Answer, Question};
use salty_vault::utils::*;
use salty_vault::vault::{account::AccountEntry, vault::Vault};
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process;
use tokio::io;
use tokio::io::Interest;
use tokio::net::UnixStream;

const SOCKET_NAME: &str = "salty.sock";

pub struct VaultInstance {
    hashed_master_pwd: String,
    vault: Vault,
}

impl VaultInstance {
    /// loads an already existant vault
    fn new() -> Self {
        let password = PasswordQuery::new("Insert Vault master password: ").read();
        let hashed_pwd = hasher::hash(&password, &Vault::salt()).unwrap();

        VaultInstance {
            hashed_master_pwd: hashed_pwd.clone(),
            vault: Vault::default(&hashed_pwd).expect("Failed loading vault!"),
        }
    }

    /// creates a new vault
    pub fn create_vault() -> Self {
        let result = PasswordQuery::new("Insert Vault master password")
            .read_and()
            .prompt("Insert one more time: ")
            .confirm_read();

        let mut hashed_pwd: String = String::from("");
        let mut pass: String = String::from("");
        let mut salt: String = String::from("");

        if let Ok(password) = result {
            salt = gen_salt().expect("Invalid salt generated!");
            hashed_pwd = hasher::hash(&password, &salt).unwrap();
            pass = password;
        }

        password::check_pass_strength(pass);
        VaultInstance {
            hashed_master_pwd: hashed_pwd.clone(),
            vault: Vault::new(&hashed_pwd, &salt),
        }
    }

    pub fn add_entry(opt: AddOpt) -> Result<(), String> {
        let AddOpt {
            site,
            user,
            hasher_salt,
        } = opt;

        let password = PasswordQuery::new("Insert site password").read();

        let entry = AccountEntry::new(&site, &user, &password);
        if let Err(entry) = self.vault.account.add(entry) {
            let answer =
                Question::new("Site already exists, do you want to overwrite it's settings?")
                    .default(Answer::NO)
                    .show_defaults()
                    .confirm();

            if answer == Answer::YES {
                self.vault.account.force_add(entry);
            }
        }
        // println!("{:#?}",account);
        Ok(())
    }

    pub fn show_entries(self) -> Result<(), String> {
        println!("{:#?}", self.vault.account); //TODO: switch to use std::fmt::Display instead of Debug

        Ok(())
    }

    pub fn check_vault_exists() {
        if !Vault::exists() {
            println!("No vault found. You need to create one first!");
            process::exit(1);
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        let vault = VaultInstance::new();
        let path_socket = dirs::home_dir()
            .expect("No home directory found in your system!")
            .join(".salty/")
            .join(SOCKET_NAME)
            .to_str()
            .expect("invalid path to socket!")
            .to_owned();

        let stream = UnixStream::connect(&path_socket).await?;
        loop {
            // wait for a command requested from CLI client

            let ready = stream.ready(Interest::READABLE).await?;
            if ready.is_readable() {
                let mut data = vec![0; 1024];
                match stream.try_read(&mut data) {
                    Ok(_) => {
                        let opt: Opt = serde_json::from_slice(&data)
                            .expect("Could not deserialize cli command!");
                        vault.run_command(opt);
                        let ready = stream.ready(Interest::WRITABLE).await?;
                        if ready.is_writable() {
                            stream.try_write(b"done!")?;
                        }
                    }
                    _ => continue,
                }
            }
        }
    }
    fn run_command(self, opt: Opt) {
        match opt {
            Opt::Generator(params) => {
                let pass = random_password(params).expect("Failed to generate random password");
                println!("{}", pass);
            }
            Opt::Create => {
                VaultInstance::create_vault();
            }
            Opt::Add(params) => {
                VaultInstance::add_entry(params);
            }
            Opt::Show => {
                VaultInstance::show_entries();
            }
            Opt::Totp => {
                Authenticator::new().validate_code();
            }
        };
        Ok(())
    }
}

pub struct CliClient {
    // vault: VaultInstance,
    socket: String,
}

impl CliClient {
    pub fn new() -> CliClient {
        let path_socket = dirs::home_dir()
            .expect("No home directory found in your system!")
            .join(".salty/")
            .join(SOCKET_NAME)
            .to_str()
            .expect("invalid path to socket!")
            .to_owned();

        let pid_file = dirs::home_dir()
            .expect("No home directory found in your system!")
            .join(".salty/")
            .join("pid");

        if !pid_file.as_path().exists() {
            ///https://docs.rs/fork/0.1.18/fork/fn.daemon.html
            if let Ok(Fork::Child) = daemon(false, true) {
                println!("Created vault process!");
                let mut file = File::create(pid_file.as_path()).expect("Failed creating Pid file");
                file.write_all(std::process::id().to_string().as_bytes());
                file.flush();
                VaultInstance::run();
            }
            std::process::exit(0); // exit cli process
        }

        CliClient {
            socket: path_socket,
        }
    }
    /// https://gist.github.com/tesaguri/b27d0d35d1a45465ddc9cb32a3ebe9ae
    /// https://docs.rs/tokio/1.13.0/tokio/net/struct.UnixStream.html
    pub async fn send_command(self, opt: Opt) -> Result<(), Box<dyn Error>> {
        let stream = UnixStream::connect(&self.socket).await?;
        let ready = stream.ready(Interest::WRITABLE).await?;
        if ready.is_writable() {
            stream.try_write(&serde_json::to_string(&opt).unwrap().into_bytes())?;
        }

        loop {
            // wait for a response from the vault daemon
            let ready = stream.ready(Interest::READABLE).await?;
            if ready.is_readable() {
                let mut data = vec![0; 1024];
                match stream.try_read(&mut data) {
                    Ok(n) => {
                        break;
                    }
                    _ => continue,
                }
            }
        }

        Ok(())
    }
}
