use crate::config::Config;
use crate::service::CommandRequest;
#[cfg(test)]
use salty_utils::testing::Testing;
use salty_utils::{security::Cipher, vault::Account};

pub struct CmdProcessor {
    vault: Option<Account>,
}
#[cfg(test)]
impl Testing for CmdProcessor {
    fn default() -> Self {
        CmdProcessor {
            vault: Some(Account::empty(Cipher::Fast)),
        }
    }
}
#[cfg(test)]
mod tests_cmd_processor {
    #[test]
    fn test_cmd_processor() {}
}

impl CmdProcessor {
    pub fn default(config: &Config) -> Self {
        CmdProcessor { vault: None }
    }

    pub fn handle_request(&self, _req: &CommandRequest) {
        unimplemented!("handle commands here")
    }
}
