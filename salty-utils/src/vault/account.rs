use crate::security::{self, Cipher};
use borsh::{BorshDeserialize, BorshSerialize};
use derivative::Derivative;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::string::String;

#[cfg(test)]
mod account_test {
    use super::*;

    #[test]
    fn test_pass_phrase() {
        let pass_phrase = MasterPassPhrase::new_helper(Cipher::Fast, "mypass", Some([0u8; 32]));
        let pass_phrase2 = MasterPassPhrase::new_helper(Cipher::Fast, "mypass", Some([0u8; 32]));
        assert_eq!(pass_phrase.master_pwd, pass_phrase2.master_pwd);

        let pass_phrase = MasterPassPhrase::new_helper(Cipher::Fast, "mypass", Some([0u8; 32]));
        let pass_phrase2 = MasterPassPhrase::new_helper(Cipher::Fast, "mypass", Some([5u8; 32]));
        assert_ne!(pass_phrase.master_pwd, pass_phrase2.master_pwd);
    }

    #[test]
    fn test_account_add() {
        let pass_phrase = MasterPassPhrase::new_helper(Cipher::Fast, "", Some([0u8; 32]));
        // use a FastCipher to speed up the test
        let mut account = Account::default("", pass_phrase);
        let entry = AccountEntry::new("google", "andrew", "123456789");
        let entry2 = AccountEntry::new("amazon", "andrew", "123456789");
        let entry3 = AccountEntry::new("facebook", "andrew", "123456789");

        assert!(account.add(entry).is_ok());
        assert!(account.add(entry2).is_ok());
        assert!(account.add(entry3).is_ok());
        assert_eq!(account.records.len(), 3);

        let entry_already_there = AccountEntry::new("facebook", "andrew", "123456789");
        assert!(account.add(entry_already_there).is_err());

        assert_eq!(account.size(), 3);
    }

    #[test]
    fn test_account_password_encryption() {
        let plain_pwd = "my plain password".to_string();
        let pass_phrase = MasterPassPhrase::new_helper(Cipher::Fast, &plain_pwd, Some([0u8; 32]));
        // use a FastCipher to speed up the test
        let mut account = Account::default("my_account", pass_phrase);

        // check account name
        assert_eq!(account.name(), "my_account");
        // check encrypted password not equal to plain password
        assert_ne!(account.password(), plain_pwd.as_bytes());
        assert_ne!(account.password().len(), plain_pwd.len());
        assert_eq!(account.password().len(), 77);

        // check the result of unencrypting the password should be equal to plain password
        let cocoon = Cipher::Fast.new(plain_pwd.as_bytes());
        let unencrypted_pwd = cocoon.unwrap(account.password()).unwrap();
        assert_eq!(unencrypted_pwd.len(), plain_pwd.len());
        assert_eq!(&unencrypted_pwd[..], plain_pwd.as_bytes());

        let entry = AccountEntry::new("google", "andrew", "123456789");

        assert!(account.add(entry).is_ok());
        assert_eq!(account.records.len(), 1);

        assert_eq!(account.size(), 1);
    }
}

pub trait Vault {
    fn name(&self) -> &str;
    fn password(&self) -> &[u8];
    fn cipher(&self) -> &Cipher;
    fn salt(&self) -> &[u8; 32];
}

impl Vault for Account {
    fn name(&self) -> &str {
        &self.name
    }
    fn password(&self) -> &[u8] {
        &self.master_pwd.master_pwd[..]
    }
    fn cipher(&self) -> &Cipher {
        &self.master_pwd.cipher
    }
    fn salt(&self) -> &[u8; 32] {
        &self.master_pwd.seed
    }
}
#[derive(Derivative, BorshSerialize, BorshDeserialize, Clone, Debug)]
#[derivative(PartialEq)]
pub struct Account {
    records: HashSet<AccountEntry>,
    name: String,
    master_pwd: MasterPassPhrase,
}

#[derive(Derivative, BorshSerialize, BorshDeserialize, Clone, Debug)]
#[derivative(PartialEq)]
pub struct MasterPassPhrase {
    master_pwd: Vec<u8>,
    cipher: Cipher,
    #[borsh_skip]
    #[derivative(PartialEq = "ignore")]
    seed: [u8; 32],
}

impl MasterPassPhrase {
    pub fn new<S: Into<String>>(cipher: Cipher, pwd: S) -> Self {
        Self::new_helper(cipher, pwd, None)
    }

    pub fn default() -> Self {
        MasterPassPhrase {
            master_pwd: Vec::with_capacity(0),
            cipher: Cipher::default(),
            seed: [0u8; 32],
        }
    }

    // use following method only for testing, it compromises security because it uses an encryption algorithm with less iterations
    pub fn default_with_fast_cipher<S: Into<String>>(pwd: S) -> Self {
        MasterPassPhrase {
            master_pwd: pwd.into().as_bytes().to_vec(),
            cipher: Cipher::Fast,
            seed: [0u8; 32],
        }
    }
    fn new_helper<S: Into<String>>(cipher: Cipher, pwd: S, seed: Option<[u8; 32]>) -> Self {
        let password = pwd.into();

        // Seed obtained by cryptographically secure random generator.
        let new_seed: [u8; 32] = seed.unwrap_or(rand::thread_rng().gen::<[u8; 32]>());

        let cocoon = cipher.with_seed(password.as_bytes(), new_seed);
        let pwd_encrypted = cocoon
            .wrap(password.as_bytes())
            .expect("Failed encrypting password!");

        MasterPassPhrase {
            master_pwd: pwd_encrypted,
            cipher: cipher,
            seed: new_seed,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Hash, Eq, PartialEq, PartialOrd, Clone, Debug)]
pub struct AccountEntry {
    record_name: String,
    user_name: String,
    pwd: String,
}

impl AccountEntry {
    pub fn new<'a>(sname: &'a str, uname: &'a str, pass: &'a str) -> Self {
        AccountEntry {
            record_name: sname.to_owned(),
            user_name: uname.to_owned(),
            pwd: pass.to_owned(),
        }
    }
}

impl Ord for AccountEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.record_name.cmp(&other.record_name)
    }
}

impl Account {
    // public members

    pub fn size(&self) -> usize {
        return self.records.len();
    }

    pub fn add(&mut self, entry: AccountEntry) -> Result<(), AccountEntry> {
        if self.records.contains(&entry) {
            return Result::Err(entry);
        }
        self.force_add(entry);
        Ok(())
    }

    pub fn force_add(&mut self, entry: AccountEntry) {
        if self.records.is_empty() {
            self.records = HashSet::new();
        }
        self.records.insert(entry);
    }

    pub fn default<S: Into<String>>(name: S, pwd: MasterPassPhrase) -> Self {
        security::generate_salt();
        Account::create(name, pwd)
    }

    pub fn empty() -> Self {
        Account::create(String::from(""), MasterPassPhrase::default())
    }

    fn create<S: Into<String>>(name: S, pwd: MasterPassPhrase) -> Self {
        Account {
            records: HashSet::new(),
            name: name.into(),
            master_pwd: pwd,
        }
    }
}
