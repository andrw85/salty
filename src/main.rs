use salty::{Opt, AddSiteOpt, Account, AccountEntry, Question, Answer, zxcvbn, Case, Casing, hasher, password};
use std::process;

fn main() {
    let opt = salty::options();       
    
    match opt {
        Opt::Generator(params) => {
            let pass = password::generate_random_password(params).expect("Failed to generate random password");
            println!("{}", pass);
        },
        Opt::AddSite(AddSiteOpt{site, user,hasher_salt}) => {
            println!("Vault master password: ");
            let password = password::get_password();
            let pw_hashed = hasher::hash(&password, &hasher_salt).unwrap();
            
            let mut account = match Account::load_from_file(&pw_hashed) {
                Ok(acc) => {
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
