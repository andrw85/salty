use crate::service::{CommandRequest, CommandResponse};
use salty_utils::{
    logs,
    security::Cipher,
    storage::HardDiskStorage,
    vault::commands::{AddCmd, Cmd, CmdErrorCode, CmdResponse, CreateCmd, LoginCmd, ShowCmd},
    vault::{Account, MasterPassPhrase},
};
use std::sync::{Arc, Mutex};

pub struct CmdProcessor {
    vault: Arc<Mutex<Account>>,
}

type GrpcCommandRp = CommandResponse;
type VaultCmdResponse = CmdResponse;

#[cfg(test)]
mod tests_cmd_processor {
    use super::*;
    use salty_utils::testing::*;
    impl Testing for CmdProcessor {
        fn default() -> Self {
            let plain_pwd = "".to_string();
            let pass = MasterPassPhrase::default_with_fast_cipher(plain_pwd);
            CmdProcessor {
                vault: Arc::new(Mutex::new(Account::default("testing", pass))),
            }
        }
    }
    #[tokio::test]
    async fn test_cmd_processor_create_vault_and_login() {
        clean_salty_home_workspace();
        let processor = <CmdProcessor as Testing>::default();
        {
            let cmd = CreateCmd {
                vault_name: "test".to_string(),
                password: "mypass".to_string(),
                local: false,
                cipher: Cipher::Fast,
            };
            let response: GrpcCommandRp = processor.handle_helper(Cmd::Create(cmd));
            let rp: CmdResponse = serde_json::from_str(&response.message).unwrap();
            let result: CmdResponse = CmdResponse {
                result: CmdErrorCode::Ok,
                message: String::from("Done!"),
            };
            assert_eq!(rp, result);
        }
        {
            let cmd = LoginCmd {
                vault_name: "test".to_string(),
                password: "mypass".to_string(),
                cipher: Cipher::Fast,
            };
            let response: GrpcCommandRp = processor.handle_helper(Cmd::Login(cmd));
            let rp: CmdResponse = serde_json::from_str(&response.message).unwrap();
            let result: CmdResponse = CmdResponse {
                result: CmdErrorCode::Ok,
                message: String::from("Login successful."),
            };
            assert_eq!(rp, result);
        }
    }
}

impl CmdProcessor {
    pub fn default() -> Self {
        CmdProcessor {
            vault: Arc::new(Mutex::new(Account::empty())),
        }
    }

    pub fn handle(&self, _req: &CommandRequest) -> GrpcCommandRp {
        let cmd: Cmd = serde_json::from_str(&_req.command).unwrap();
        logs::debug!("CmdProcessor received command request", cmd);
        let response = self.handle_helper(cmd);
        logs::debug!("CmdProcessor returning response", response);
        response
    }
    fn handle_helper(&self, cmd: Cmd) -> GrpcCommandRp {
        let account = &mut *self.vault.lock().unwrap();

        let rp: VaultCmdResponse = match cmd {
            Cmd::Create(c) => c.execute(account),
            Cmd::Login(c) => c.execute(account),
            Cmd::Add(c) => c.execute(account),
            Cmd::Show(c) => c.execute(account),
        };
        GrpcCommandRp {
            message: serde_json::to_string(&rp).unwrap(),
        }
    }
}

trait Executor {
    fn execute(&self, account: &mut Account) -> VaultCmdResponse;
}

impl Executor for CreateCmd {
    fn execute(&self, account: &mut Account) -> VaultCmdResponse {
        let pass_phrase = MasterPassPhrase::new(self.cipher.clone(), self.password.to_owned());
        *account = Account::default(self.vault_name.to_owned(), pass_phrase);
        match account.exists() {
            false => {
                // failed loading because no account with same name, create one
                account.store_to_disk();
                return VaultCmdResponse {
                    result: CmdErrorCode::Ok,
                    message: String::from("Done!"),
                };
            }
            true => VaultCmdResponse {
                result: CmdErrorCode::AccountAlreadyExists,
                message: String::from(
                    "Cannot create the account, there is already one with the same name.",
                ),
            },
        }
    }
}

impl Executor for LoginCmd {
    fn execute(&self, account: &mut Account) -> VaultCmdResponse {
        match Account::load_from_disk(self.cipher.clone(), &self.vault_name, &self.password) {
            Ok(acc) => {
                *account = acc;
                return VaultCmdResponse {
                    result: CmdErrorCode::Ok,
                    message: String::from("Login successful."),
                };
            }
            Err(_) => {
                return VaultCmdResponse {
                    result: CmdErrorCode::StorageBackendError,
                    message: String::from("Failed loading account from disk!"),
                }
            }
        }
    }
}

impl Executor for AddCmd {
    fn execute(&self, _account: &mut Account) -> VaultCmdResponse {
        println!("AddCmd");
        unimplemented!("")
    }
}

impl Executor for ShowCmd {
    fn execute(&self, _account: &mut Account) -> VaultCmdResponse {
        println!("ShowCmd");
        unimplemented!("")
    }
}
