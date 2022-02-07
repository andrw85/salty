mod options;
mod password;
mod hasher;
mod storage;
use options::options;
use options::{Opt, ManagerOpt,PasswordGenOpt, AddSiteOpt};
use storage::{Account, AccountEntry};
use password::get_password;
use question::{Answer,Question};
use std::process;
use zxcvbn::zxcvbn;
use convert_case::{Case, Casing};
use strum_macros::Display;

fn main() {
    let opt = options();       
    // let pw = password::gen(opt).unwrap();
    match opt {
        Opt::Generator(PasswordGenOpt{ hasher_salt, .. }) => {

            let pw = "holahola".to_string() + &hasher_salt;
            let pw_hashed = hasher::hash(&pw, &hasher_salt).unwrap();
            
            println!("pwd: {:?}", &pw);
            println!("hash: {:?}", &pw_hashed);
        },
        Opt::AddSite(AddSiteOpt{site, user,hasher_salt}) => {
            println!("Vault master password: ");
            let password = password::get_password();
            let pw_hashed = hasher::hash(&password, &hasher_salt).unwrap();
            
            let mut account = match Account::load_from_file(&pw_hashed) {
                Ok(mut acc) => {
                    acc
                },
                Err(cocoon::Error::Cryptography) => {
                    eprintln!("Invalid vault master password!");
                    process::exit(1);
                },
                _ => {
                    let estimate = zxcvbn(&password, &[]).expect("Failed checking password strength");
                    if estimate.score() < 3{
                        let feedback = estimate.feedback().as_ref().expect("Password too weak, failed getting feedback!");
                        eprintln!("Error: weak password, {}", feedback.warning().unwrap().to_string().to_case(Case::Lower));
                        process::exit(1);
                    }
                    println!("Creating new account!");
                    Account::new(&pw_hashed)
                }
            };

            println!("Insert site password: ");
            let password = password::get_password();

            let entry = AccountEntry::new(&site, &user, &password);
            if let Err(entry) = account.add(entry) {
                let answer = Question::new("Site already exists, do you want to overwrite it's settings?")
                            .default(Answer::NO)
                            .show_defaults()
                            .confirm();
            
                if answer == Answer::YES {
                    account.force_add(entry);
                } 
            }
            println!("{:#?}",account);
            
        }
    }
}
