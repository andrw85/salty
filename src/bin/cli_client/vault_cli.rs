use salty::utils::*;
use salty::vault::{vault::{Vault}, account::{AccountEntry}};
use super::options::{AddSiteOpt};
use question::{Answer,Question};

pub fn add_entry(opt: AddSiteOpt) -> Result<(), String> {
    let AddSiteOpt{site, user,hasher_salt} = opt;

    if !Vault::exists() {
        return Err("No vault, you need to create one first!".to_string())
    }
    let password = PasswordQuery::new("Insert Vault master password: ").read();
    let pw_hashed = hasher::hash(&password, &hasher_salt).unwrap();
    
    let mut vault = match Vault::default(&pw_hashed) {
        Ok(acc) => {
            acc
        },
        Err(cocoon::Error::Cryptography) => {
            return Err("Invalid vault master password!".to_string());
        },
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

pub fn create_vault() -> Result<(), String> {
    let result = PasswordQuery::new("Insert Vault master password").read_and().prompt("Insert one more time: ").confirm_read();
    
    if let Ok(password) = result {
        let pw_hashed = hasher::hash(&password, "aot/I3YepRSH5AaZe+oDEQ").unwrap(); //TODO: make salt configurable
    
        password::check_pass_strength(password)?;
        Vault::new(&pw_hashed);
    }
    Ok(())
}

pub fn show_entries() -> Result<(), String> {
    if !Vault::exists() {
        return Err("No vault, you need to create one first!".to_string())
    }

    let password = PasswordQuery::new("Insert Vault master password").read();
    let pw_hashed = hasher::hash(&password, "aot/I3YepRSH5AaZe+oDEQ").unwrap(); //TODO: make salt configurable
    
    let vault = match Vault::default(&pw_hashed) {
        Ok(acc) => {
            acc
        },
        Err(cocoon::Error::Cryptography) => {
            return Err("Invalid vault master password!".to_string());
        },
        _ => {
            return Err("Unknown error adding an entry.".to_string());
        }
    };

    println!("{:#?}", vault.account); //TODO: switch to use std::fmt::Display instead of Debug

    Ok(())
}