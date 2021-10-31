pub mod generator;
pub mod options;
pub mod vault_cli;

pub use generator::{random_password, RandomPassword};
pub use options::{options, AddOpt, FlagsOpt, ManagerOpt, Opt, PasswordGenOpt};
use std::process;
pub use vault_cli::add_entry;

use salty_vault::utils::authenticator::Authenticator;

fn main() {
    let opt = options::options();
    let result: Result<(), String> = match opt {
        Opt::Generator(params) => {
            let pass =
                generator::random_password(params).expect("Failed to generate random password");
            println!("{}", pass);
            Ok(())
        }
        Opt::Create => vault_cli::create_vault(),
        Opt::Add(params) => vault_cli::add_entry(params),
        Opt::Show => vault_cli::show_entries(),
        Opt::Totp => {
            Authenticator::new().validate_code();
            Ok(())
        }
        Opt::None => {
            // println!("Default vault in ~/.salty/");
            // vault_cli::show_entries()
            Ok(())
        }
    };

    if let Err(msg) = result {
        eprintln!("{}", msg);
        process::exit(1);
    }
}
