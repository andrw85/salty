use super::options::PasswordGenOpt;
use passwords::PasswordGenerator;
use salty::utils::hasher;
use std::fmt;

static PG: PasswordGenerator = PasswordGenerator {
    length: 63,
    numbers: true,
    lowercase_letters: true,
    uppercase_letters: true,
    symbols: false,
    spaces: false,
    exclude_similar_characters: true,
    strict: true,
};

pub struct RandomPassword {
    hash: String,
    plaintext: String,
}

impl fmt::Display for RandomPassword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.plaintext)
    }
}

pub fn random_password(opt: PasswordGenOpt) -> Result<RandomPassword, &'static str> {
    let pg = PasswordGenerator {
        length: opt.length,
        numbers: opt.numbers,
        lowercase_letters: opt.lowercase_letters,
        uppercase_letters: opt.uppercase_letters,
        symbols: opt.symbols,
        spaces: opt.spaces,
        exclude_similar_characters: opt.exclude_similar_characters,
        strict: opt.strict,
    };
    let mut result = RandomPassword {
        hash: "".to_owned(),
        plaintext: "".to_owned(),
    };
    result.plaintext = pg
        .generate_one()
        .expect("Failed generating random password!");
    let PasswordGenOpt { hasher_salt, .. } = opt;
    result.hash = hasher::hash(&result.plaintext, &hasher_salt)?;

    Ok(result)
}

pub fn gen_salt() -> Result<String, &'static str> {
    PG.generate_one()
}
