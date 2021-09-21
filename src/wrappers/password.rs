use rpassword;
use zxcvbn::zxcvbn;
use convert_case::{Case, Casing};
use std::process;

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

pub fn check_pass_strength(password: String) -> Result<(), String> {
    let estimate = zxcvbn(&password, &[]).expect("Failed checking password strength");
    if estimate.score() < 3{
        let feedback = estimate.feedback().as_ref().expect("Password too weak, failed getting feedback!");
        // eprintln!("Error: weak password, {}", feedback.warning().unwrap().to_string().to_case(Case::Lower));
        let human_readable_error = format!("Error: weak password, {}", feedback.warning().unwrap().to_string().to_case(Case::Lower));
        return Err(human_readable_error);
    }
    Ok(())
}