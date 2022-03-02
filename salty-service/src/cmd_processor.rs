use crate::config::Config;
use crate::service::{CommandRequest, CommandResponse};
use salty_utils::{
    vault::commands::{AddCmd, Cmd, CreateCmd, LoginCmd, ShowCmd},
    vault::Account,
};
use std::sync::{Arc, Mutex};

pub struct CmdProcessor {
    vault: Arc<Mutex<Account>>,
}

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

    pub fn handle(&self, _req: &CommandRequest) -> CommandResponse {
        let cmd: Cmd = serde_json::from_str(&_req.command).unwrap();
        self.handle_helper(cmd)
    }
    fn handle_helper(&self, cmd: Cmd) -> CommandResponse {
        let account = &mut *self.vault.lock().unwrap();
        let message = match cmd {
            Cmd::Create(c) => c.execute(account),
            Cmd::Login(c) => c.execute(account),
            Cmd::Add(c) => c.execute(account),
            Cmd::Show(c) => c.execute(account),
        };
        CommandResponse { message: message }
    }
}

trait Executor {
    fn execute(&self, account: &mut Account) -> String;
}

impl Executor for CreateCmd {
    fn execute(&self, _account: &mut Account) -> String {
        unimplemented!("")
    }
}

impl Executor for LoginCmd {
    fn execute(&self, _account: &mut Account) -> String {
        unimplemented!("")
    }
}

impl Executor for AddCmd {
    fn execute(&self, _account: &mut Account) -> String {
        unimplemented!("")
    }
}

impl Executor for ShowCmd {
    fn execute(&self, _account: &mut Account) -> String {
        unimplemented!("")
    }
}
