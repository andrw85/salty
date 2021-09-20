use borsh::{BorshSerialize, BorshDeserialize};
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use std::fs::File;
pub use cocoon::{Cocoon, Error};
use std::result::Result::{Err};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct Account {
    sites: HashSet<AccountEntry>,
    master_pwd: String,
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
    pub fn new<'a>(master_password: &'a str) -> Self {
        Account {
            sites: HashSet::new(),
            master_pwd: master_password.to_owned(),
        }
    }

    pub fn load_from_file<'a>(master_pwd: &'a str) -> Result<Self,cocoon::Error> {
        let mut file = File::open("target/test.db")?;
        let metadata = file.metadata().expect("Could not obtain file attributes");
        let mut permissions = metadata.permissions();
        permissions.set_readonly(false);

        let cocoon = Cocoon::new(&master_pwd.as_bytes());
        let encoded_data = cocoon.parse(&mut file)?;
        let res = Self::try_from_slice(&encoded_data)?;
        Ok(res)
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

impl Drop for Account {
    fn drop(&mut self) {
        let encoded_account =self.try_to_vec().unwrap();

        let mut file = File::create("target/test.db").expect("Could not create db file.");
        
        let cocoon = Cocoon::new(&self.master_pwd.as_bytes());
        // Dump the serialized database into a file as an encrypted container.
        cocoon.dump(encoded_account, &mut file).expect("Could not dump encrpyted data into db file.");

        // make file only readable
        let metadata = file.metadata().expect("Could not obtain file attributes");
        let mut permissions = metadata.permissions();
        permissions.set_readonly(true);
    }
}

