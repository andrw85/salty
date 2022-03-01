use crate::config::Config;
use crate::service::{CommandRequest, CommandResponse};
use salty_utils::{vault::commands::Cmd, vault::Account};

pub struct CmdProcessor {
    vault: Option<Account>,
}

#[cfg(test)]
mod tests_cmd_processor {
    use super::{Account, Cmd, CmdProcessor, InternalCmd};
    use salty_utils::{security::Cipher, testing::Testing};
    impl Testing for CmdProcessor {
        fn default() -> Self {
            CmdProcessor {
                vault: Some(Account::empty(Cipher::Fast)),
            }
        }
    }
    #[test]
    #[should_panic(expected = "not implemented: ")]
    fn test_cmd_processor_create_vault() {
        let processor = <CmdProcessor as Testing>::default();
        let cmd = Cmd::Create {
            vault_name: "test".to_string(),
        };
        processor.create(cmd);
    }
}

impl CmdProcessor {
    pub fn default(_config: &Config) -> Self {
        CmdProcessor { vault: None }
    }

    pub fn handle(&self, _req: &CommandRequest) -> CommandResponse {
        let cmd: Cmd = serde_json::from_str(&_req.command).unwrap();

        let message = match cmd {
            Cmd::Create { vault_name: _ } => self.create(cmd),
            Cmd::Login { vault_name: _ } => self.login(cmd),
            Cmd::Add(_) => self.add(cmd),
            Cmd::Show => self.show(cmd),
        };
        CommandResponse { message: message }
    }
}

trait InternalCmd {
    fn create(&self, cmd: Cmd) -> String;
    fn login(&self, cmd: Cmd) -> String;
    fn add(&self, cmd: Cmd) -> String;
    fn show(&self, cmd: Cmd) -> String;
}

impl InternalCmd for CmdProcessor {
    fn create(&self, _cmd: Cmd) -> String {
        unimplemented!("")
    }
    fn login(&self, _cmd: Cmd) -> String {
        unimplemented!("")
    }
    fn add(&self, _cmd: Cmd) -> String {
        unimplemented!("")
    }
    fn show(&self, _cmd: Cmd) -> String {
        unimplemented!("")
    }
}
