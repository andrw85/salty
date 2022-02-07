use borsh::{BorshSerialize, BorshDeserialize};
use std::collections::{HashSet};
use std::cmp::Ordering;
use std::fs::File;
pub use cocoon::{Cocoon, Error};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct Account {
    sites: HashSet<AccountEntry>
}
 
#[derive(BorshSerialize, BorshDeserialize, Hash, Eq, PartialEq, PartialOrd, Clone, Debug)]
pub struct AccountEntry {
    site_name: String,
    user_name: String,
    pwd: String,
}

impl AccountEntry {
    pub fn new<'a>(sname: &'a str, uname: &'a str, pass: &'a str) -> Self {
        AccountEntry {
            site_name: sname.to_owned(),
            user_name: uname.to_owned(),
            pwd: pass.to_owned(),
        }
    }
}

impl Ord for AccountEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.site_name.cmp(&other.site_name)
    }
}

impl Account {
    pub fn new() -> Self {
        Account {
            sites: HashSet::new(),
        }
    }
    
    pub fn size(&self) -> usize {
        return self.sites.len();
    }

    pub fn add(&mut self, entry: AccountEntry) -> Result<(), AccountEntry> {
        if self.sites.contains(&entry) {
            return Result::Err(entry);
        }
        self.force_add(entry);
        Ok(())
    }

    pub fn force_add(&mut self, entry: AccountEntry){
        if self.sites.is_empty() {
            self.sites = HashSet::new();
        }
        self.sites.insert(entry);
    }
}

