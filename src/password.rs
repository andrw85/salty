use passwords::PasswordGenerator;
use super::options;
use rpassword;
use std::error::Error;
use zxcvbn::zxcvbn;
use convert_case::{Case, Casing};
use strum_macros::Display;
use std::process;

pub fn gen(parameters : options::Opt) -> Result<String, &'static str> { 
    if let options::Opt::Generator(opt) = parameters {  
        let pg  = PasswordGenerator {
                length: opt.length,
                numbers: opt.numbers,
                lowercase_letters: opt.lowercase_letters,
                uppercase_letters: opt.uppercase_letters,
                symbols: opt.symbols,
                spaces: opt.spaces,
                exclude_similar_characters: opt.exclude_similar_characters,
                strict: opt.strict,
            };

        pg.generate_one()
    } else {
        Err("Failed generating password")
    }
}        

pub fn get_password() -> String {
    rpassword::read_password().expect("Failed getting a password")
}

pub fn ask_for_new_password(password: String) -> String {
    rpassword::read_password().expect("Failed getting a password");
    let estimate = zxcvbn(&password, &[]).expect("Failed checking password strength");
    if estimate.score() < 3{
        let feedback = estimate.feedback().as_ref().expect("Password too weak, failed getting feedback!");
        eprintln!("Error: weak password, {}", feedback.warning().unwrap().to_string().to_case(Case::Lower));
        process::exit(1);
    }
    password
}
