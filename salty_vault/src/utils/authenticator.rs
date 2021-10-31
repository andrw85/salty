use super::password::PasswordQuery;
use boringauth::oath::{TOTPBuilder, TOTP};

pub struct Authenticator(TOTP);

impl Authenticator {
    pub fn new() -> Self {
        match PasswordQuery::new("Please enter your master password for authentication to begin:")
            .read_and()
            .prompt("Enter your password once more: ")
            .confirm_read()
        {
            Ok(key) => {
                let totp = TOTPBuilder::new().ascii_key(&key).finalize().unwrap();
                let code = totp.generate();
                assert_eq!(code.len(), 6);
                println!("Authentication code: {}", code);
                return Authenticator(totp);
            }
            Err(e) => panic!("{}", e),
        }
    }

    pub fn validate_code(&self) {
        let code = PasswordQuery::new("Insert one-time-password:").read();
        if !self.0.is_valid(&code) {
            eprintln!("Authentication code is not valid!");
            return;
        }
        println!("Valid code!");
    }
}
