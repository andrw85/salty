use super::options::{PasswordGenOpt};
use super::hasher;
use passwords::PasswordGenerator;
use rpassword;
use zxcvbn::zxcvbn;
use convert_case::{Case, Casing};
use std::fmt;
use std::process;

pub struct RandomPassword {
    hash: String,
    plaintext: String,
}

impl fmt::Display for RandomPassword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.plaintext)
    }
}

pub fn generate_random_password(opt : PasswordGenOpt) -> Result<RandomPassword, &'static str> { 
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
    let mut result = RandomPassword {hash: "".to_owned(), plaintext: "".to_owned()};
    result.plaintext = pg.generate_one().expect("Failed generating random password!");    
    let PasswordGenOpt{ hasher_salt, .. } = opt;
    result.hash = hasher::hash(&result.plaintext, &hasher_salt)?;

    Ok(result)
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
