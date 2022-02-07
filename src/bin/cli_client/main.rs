pub mod generator;
pub mod vault_cli;
pub mod options;

pub use generator::{RandomPassword, random_password};
pub use vault_cli::{add_entry};
pub use options::{options, Opt, PasswordGenOpt, FlagsOpt, ManagerOpt, AddSiteOpt};
use std::process;

fn main() {
    let opt = options::options();
    
    let result: Result<(), String> = match opt {
        Opt::Generator(params) => {
            let pass = generator::random_password(params).expect("Failed to generate random password");
            println!("{}", pass);
            Ok(())
        },
        Opt::CreateVault => {
            vault_cli::create_vault()
        }
        Opt::AddSite(params) => {
            vault_cli::add_entry(params)
        },
        Opt::ShowEntries => {
            vault_cli::show_entries()
        },
    };

    if let Err(msg) = result {
        eprintln!("{}", msg);
        process::exit(1);
    }
}
