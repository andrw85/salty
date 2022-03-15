use borsh::{BorshDeserialize, BorshSerialize};
use cocoon::{Cocoon, Error};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub enum Cipher {
    Fast,
    Slow,
}
impl Default for Cipher {
    fn default() -> Self {
        Cipher::Slow
    }
}

impl Cipher {
    pub fn new<'a>(&self, pwd: &'a [u8]) -> Cocoon<'a, cocoon::Creation> {
        match self {
            Cipher::Fast => Cocoon::new(pwd).with_weak_kdf(),
            Cipher::Slow => Cocoon::new(pwd),
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
    pub fn hash<'a>(&self, thing: &'a [u8]) -> Result<Vec<u8>, Error> {
        match self {
            Cipher::Fast => return Cocoon::new(thing).with_weak_kdf().wrap(thing),
            Cipher::Slow => return Cocoon::new(thing).wrap(thing),
        }
    }
}
