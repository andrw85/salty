use borsh::{BorshDeserialize, BorshSerialize};
pub use cocoon::{Cocoon, Error};
use std::cmp::Ordering;
use std::collections::HashSet;

#[cfg(test)]
mod account_test {
    use super::{Account, AccountEntry};

    #[test]
    fn test_account_add() {
        let mut account = Account::new();
        let entry = AccountEntry::new("google", "andrew", "123456789");
        let entry2 = AccountEntry::new("amazon", "andrew", "123456789");
        let entry3 = AccountEntry::new("facebook", "andrew", "123456789");

        assert!(account.add(entry).is_ok());
        assert!(account.add(entry2).is_ok());
        assert!(account.add(entry3).is_ok());
        assert_eq!(account.sites.len(), 3);

        let entry_already_there = AccountEntry::new("facebook", "andrew", "123456789");
        assert!(account.add(entry_already_there).is_err());

        assert_eq!(account.size(), 3);
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct Account {
    sites: HashSet<AccountEntry>,
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

    pub fn force_add(&mut self, entry: AccountEntry) {
        if self.sites.is_empty() {
            self.sites = HashSet::new();
        }
        self.sites.insert(entry);
    }
}
