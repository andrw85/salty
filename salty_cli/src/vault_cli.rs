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
use std::io::Write;
use std::path::Path;
use std::process;
use tokio::io::Interest;
use tokio::net::UnixStream;
use tokio::runtime::Runtime;

const SOCKET_NAME: &str = "salty.sock";

pub struct VaultDaemon {
    hashed_master_pwd: String,
    vault: Vault,
}

impl VaultDaemon {
    /// loads an already existant vault
    fn load() -> Self {
        let password = PasswordQuery::new("Insert Vault master password: ").read();
        let hashed_pwd = hasher::hash(&password, &Vault::salt()).unwrap();

        VaultDaemon {
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
        VaultDaemon {
            hashed_master_pwd: hashed_pwd.clone(),
            vault: Vault::new(&hashed_pwd, &salt),
        }
    }

    pub fn add_entry(&mut self, opt: AddOpt) -> Result<(), String> {
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

    pub fn show_entries(&self) -> Result<(), String> {
        println!("{:#?}", self.vault.account); //TODO: switch to use std::fmt::Display instead of Debug

        Ok(())
    }

    pub fn check_vault_exists() {
        if !Vault::exists() {
            println!("No vault found. You need to create one first!");
            process::exit(1);
        }
    }

    pub fn run() -> Result<(), Box<dyn Error>> {
        let mut daemon = {
            if Vault::exists() {
                VaultDaemon::load()
            } else {
                VaultDaemon::create_vault()
            }
        };
        // Create the runtime
        let rt = Runtime::new()?;
        // Spawn a future onto the runtime
        rt.spawn(async move {
            daemon.run_loop().await;
        });
        Ok(())
    }

    async fn run_loop(&mut self) -> Result<(), Box<dyn Error>> {
        let default_dir = dirs::home_dir()
            .expect("No home directory found in your system!")
            .join(".salty/");
        let path_socket = default_dir
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
                        self.run_command(opt);
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

    fn run_command(&mut self, opt: Opt) {
        match opt {
            Opt::Generator(params) => {
                let pass = random_password(params).expect("Failed to generate random password");
                println!("{}", pass);
            }
            Opt::Create => {
                VaultDaemon::create_vault();
            }
            Opt::Add(params) => {
                self.add_entry(params);
            }
            Opt::Show => {
                self.show_entries();
            }
            Opt::Totp => {
                Authenticator::new().validate_code();
            }
        };
    }
}
