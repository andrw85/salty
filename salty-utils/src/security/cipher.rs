use borsh::{BorshDeserialize, BorshSerialize};
use cocoon::Cocoon;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub enum Cipher {
    Fast,
    Slow,
}

impl Cipher {
    pub fn new<'a>(&self, pwd: &'a String) -> Cocoon<'a, cocoon::Creation> {
        match self {
            Cipher::Fast => Cocoon::new(pwd.as_bytes()).with_weak_kdf(),
            Cipher::Slow => Cocoon::new(pwd.as_bytes()),
        }
    }
    pub fn from_bytes<'a>(&self, pwd: &'a [u8]) -> Cocoon<'a, cocoon::Creation> {
        match self {
            Cipher::Fast => Cocoon::new(pwd).with_weak_kdf(),
            Cipher::Slow => Cocoon::new(pwd),
        }
    }
    pub fn with_seed<'a>(&self, pwd: &'a [u8], seed: [u8; 32]) -> Cocoon<'a, cocoon::Creation> {
        match self {
            Cipher::Fast => Cocoon::from_seed(pwd, seed).with_weak_kdf(),
            Cipher::Slow => Cocoon::from_seed(pwd, seed),
        }
    }
}
