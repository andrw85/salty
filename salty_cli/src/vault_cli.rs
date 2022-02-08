use super::generator::gen_salt;
use super::options::AddOpt;
use question::{Answer, Question};
use salty_vault::utils::*;
use salty_vault::vault::{account::AccountEntry, vault::Vault};

struct VaultCli {
    vault: Vault,
}

impl VaultCli {
    pub fn load_vault(v: &mut Vault) -> Self {
        VaultCli { vault: v }
    }
}

pub fn add_entry(opt: AddOpt) -> Result<(), String> {
    let AddOpt {
        site,
        user,
        hasher_salt,
    } = opt;

    if !Vault::exists() {
        return Err("No vault, you need to create one first!".to_string());
    }
    let password = PasswordQuery::new("Insert Vault master password: ").read();
    let pw_hashed = hasher::hash(&password, &Vault::salt()).unwrap();

    let mut vault = match Vault::default(&pw_hashed) {
        Ok(acc) => acc,
        Err(cocoon::Error::Cryptography) => {
            return Err("Invalid vault master password!".to_string());
        }
        _ => {
            return Err("Unknown error adding an entry.".to_string());
        }
    };

    let password = PasswordQuery::new("Insert site password").read();

    let entry = AccountEntry::new(&site, &user, &password);
    if let Err(entry) = vault.account.add(entry) {
        let answer = Question::new("Site already exists, do you want to overwrite it's settings?")
            .default(Answer::NO)
            .show_defaults()
            .confirm();

        if answer == Answer::YES {
            vault.account.force_add(entry);
        }
    }
    // println!("{:#?}",account);
    Ok(())
}

pub fn create_vault(name: &str) -> Result<(), String> {
    println!("Creating vault: {}", name);

    let result = PasswordQuery::new("Insert Vault master password")
        .read_and()
        .prompt("Insert one more time: ")
        .confirm_read();

    if let Ok(password) = result {
        let salt = gen_salt().expect("Invalid salt generated!");
        let pw_hashed = hasher::hash(&password, &salt).unwrap();

        password::check_pass_strength(password)?;
        Vault::new(&name, &pw_hashed, &salt);
    }
    Ok(())
}

pub fn show_entries() -> Result<(), String> {
    println!("{:#?}", vault); //TODO: switch to use std::fmt::Display instead of Debug

    Ok(())
}

// add_entries and show_entries:
// 1. if not already running process then
// 1a Ask for name of vault.
// 1b Check if vault exists and if it does not return error
// 1c Ask for master password of vault
// 1d Start salty process loading the vault into the process
// 2. run command in the process

// create a new vault:
// 1. if no process is running, then
// 2. ask for name of vault if not yet provided
// 3. ask for master password of vault if not yet provided
// 4. create the empty vault
// 5. start new salty process
// 6. load vault into that process (for future show and add entry commands)
