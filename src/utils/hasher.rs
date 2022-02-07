use pbkdf2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier},
    Pbkdf2,
};

pub fn hash(password: &str, salt: &str) -> Result<String, &'static str> {
    let pwd = password.as_bytes();
    // let salt = SaltString::generate(&mut OsRng);
    // let salt = salt.as_bytes();
    // println!("salt {}", &salt.as_str());
    // Hash pwd to PHC string ($pbkdf2-sha256$...)
    let password_hash = Pbkdf2.hash_password(pwd, &salt).unwrap().to_string();

    // Verify pwd against PHC string
    let parsed_hash = PasswordHash::new(&password_hash).unwrap();
    assert!(Pbkdf2.verify_password(pwd, &parsed_hash).is_ok());
    Ok(password_hash)
}
