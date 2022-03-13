use crate::config::Config;
use crate::service::{CommandRequest, CommandResponse};
use salty_utils::{
    logs,
    storage::{HardDiskStorage, StorageError},
    vault::commands::{AddCmd, Cmd, CmdErrorCode, CmdResponse, CreateCmd, LoginCmd, ShowCmd},
    vault::Account,
};
use std::sync::{Arc, Mutex};

pub struct CmdProcessor {
    vault: Arc<Mutex<Account>>,
}

type GrpcCommandRp = CommandResponse;
type VaultCmdResponse = CmdResponse;

#[cfg(test)]
mod tests_cmd_processor {
    use super::{Account, Arc, Cmd, CmdProcessor, CreateCmd, Mutex};
    use salty_utils::{security::Cipher, testing::Testing};
    impl Testing for CmdProcessor {
        fn default() -> Self {
            CmdProcessor {
                vault: Arc::new(Mutex::new(Account::empty(Cipher::Fast))),
            }
        }
    }
    #[test]
    #[should_panic(expected = "not implemented: ")]
    fn test_cmd_processor_create_vault() {
        let processor = <CmdProcessor as Testing>::default();
        let cmd = CreateCmd {
            vault_name: "test".to_string(),
        };
        processor.handle_helper(Cmd::Create(cmd));
    }
}

impl CmdProcessor {
    pub fn default(config: &Config) -> Self {
        CmdProcessor {
            vault: Arc::new(Mutex::new(Account::empty(config.cipher.clone()))),
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
        *account = Account::default(self.vault_name.to_owned(), self.password.to_owned());
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
        *account = Account::default(self.vault_name.to_owned(), self.password.to_owned());
        match account.exists() {
            false => VaultCmdResponse {
                result: CmdErrorCode::AccountDoesNotExist,
                message: String::from("Login failed! Account does not server side."),
            },
            true => match account.load_from_disk() {
                Ok(_) => VaultCmdResponse {
                    result: CmdErrorCode::Ok,
                    message: String::from("Login successful."),
                },
                Err(_) => VaultCmdResponse {
                    result: CmdErrorCode::StorageBackendError,
                    message: String::from("Failed loading account from disk!"),
                },
            },
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
