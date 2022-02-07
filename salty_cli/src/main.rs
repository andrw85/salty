pub mod generator;
pub mod options;
pub mod vault_cli;

use std::env;
// pub use generator::{random_password, RandomPassword};
// pub use options::{options, AddOpt, FlagsOpt, ManagerOpt, Opt, PasswordGenOpt};
// use salty_vault::utils::authenticator::Authenticator;
use options::options;
use std::process;
use vault_cli::CliClient;

#[tokio::main]
async fn main() {
    let vault_cli = CliClient::new();

    if env::args_os().len() == 1 {
        // when no CLI arguments
        // println!("Default vault in ~/.salty/");
        return;
    }

    let opt = options::options();
    vault_cli.send_command(opt);
    // let result: Result<(), String> = match opt {
    //     Opt::Generator(params) => {
    //         let pass =
    //             generator::random_password(params).expect("Failed to generate random password");
    //         println!("{}", pass);
    //         Ok(())
    //     }
    //     Opt::Create => vault_cli::create_vault(),
    //     Opt::Add(params) => vault_cli::add_entry(params),
    //     Opt::Show => vault_cli::show_entries(),
    //     Opt::Totp => {
    //         Authenticator::new().validate_code();
    //         Ok(())
    //     }
    // };

    // if let Err(msg) = result {
    //     eprintln!("{}", msg);
    //     process::exit(1);
    // }
}
