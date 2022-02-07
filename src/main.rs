use salty::options::{Opt};
use salty::cli_commands::{generator, vault_cli};
use std::process;

fn main() {
    let opt = salty::options::options();       
    
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
