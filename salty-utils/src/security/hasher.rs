use crate::logs;
use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Pbkdf2,
};
use std::fs;
use std::path::PathBuf;

pub fn generate_salt() {
    //
    // Save vault data in disk
    //
    let salt_path = dirs::home_dir()
        .expect("No home directory found in your system!")
        .join(".salty/")
        .join("salty")
        .with_extension("salt");
    let path = PathBuf::from(&salt_path);
    if path.is_file() {
        return;
    }

    logs::debug!("Generating general salt...");
    let salt = SaltString::generate(&mut OsRng);

    fs::write(salt_path, salt.as_bytes()).expect("Could not write salt!");
    logs::debug!("Generated...");
}

fn load_salt() -> SaltString {
    logs::debug!("Loading general salt...");
    let salt_path = dirs::home_dir()
        .expect("No home directory found in your system!")
        .join(".salty/")
        .join("salty")
        .with_extension("salt");
    let data = fs::read(salt_path).unwrap();
    SaltString::b64_encode(&data).unwrap()
}

pub fn hash(password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let pwd = password.as_bytes();
    let salt = load_salt();
    let password_hash = Pbkdf2.hash_password(pwd, &salt).unwrap();

    // generate a string without containing forward slashes:
    Ok(password_hash.hash.unwrap().to_string().replace("/", ""))
}
