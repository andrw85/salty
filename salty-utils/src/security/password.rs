use convert_case::{Case, Casing};
use rpassword;
use std::process;
use zxcvbn::zxcvbn;

pub struct PasswordQuery {
    pub password: String,
}

impl PasswordQuery {
    pub fn new(help_msg: &'static str) -> Self {
        println!("{}", help_msg);
        PasswordQuery {
            password: String::default(),
        }
    }

    pub fn prompt(&self, msg: &'static str) -> &Self {
        println!("{}", msg);
        self
    }

    pub fn read(&mut self) -> String {
        self.password = rpassword::read_password().expect("Failed getting a password");
        self.password.clone()
    }
    pub fn read_and(&mut self) -> &Self {
        self.password = rpassword::read_password().expect("Failed getting a password");
        self
    }

    pub fn confirm_read(&self) -> Result<String, String> {
        let pass = rpassword::read_password().expect("Failed getting a password");
        if pass == self.password {
            return Ok(pass);
        }
        Err("Passwords don't match".to_string())
    }
}

pub fn check_pass_strength(password: String) -> Result<(), String> {
    let estimate = zxcvbn(&password, &[]).expect("Failed checking password strength");
    if estimate.score() < 3 {
        let feedback = estimate
            .feedback()
            .as_ref()
            .expect("Password too weak, failed getting feedback!");
        // eprintln!("Error: weak password, {}", feedback.warning().unwrap().to_string().to_case(Case::Lower));
        let human_readable_error = format!(
            "Error: weak password, {}",
            feedback.warning().unwrap().to_string().to_case(Case::Lower)
        );
        return Err(human_readable_error);
    }
    Ok(())
}

pub fn ask_for_new_password(password: String) -> String {
    rpassword::read_password().expect("Failed getting a password");
    let estimate = zxcvbn(&password, &[]).expect("Failed checking password strength");
    if estimate.score() < 3 {
        let feedback = estimate
            .feedback()
            .as_ref()
            .expect("Password too weak, failed getting feedback!");
        eprintln!(
            "Error: weak password, {}",
            feedback.warning().unwrap().to_string().to_case(Case::Lower)
        );
        process::exit(1);
    }
    password
}
