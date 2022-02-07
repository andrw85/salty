use super::account::Account;
use borsh::{BorshSerialize, BorshDeserialize};
use cocoon::{Cocoon, Error};
use std::fs::File;
use std::path;

struct Vault {
    master_pwd: String,
    account: Account, 
    file_path: String,
}

impl Vault {
    pub fn default<'a>(master_pwd: &'a str) -> Self {
        Vault {
            master_pwd: master_pwd.to_owned(),
            account:  Account::new(),
            file_path: "target/test.db".to_owned(),
        }        
    }

    
    pub fn from_file<'a,'b>(master_password: &'a str, path: &'b str) -> Result<Self,cocoon::Error> {
        let mut file = File::open(&path)?;
        let metadata = file.metadata()?;
        let mut permissions = metadata.permissions();
        permissions.set_readonly(false);

        let cocoon = Cocoon::new(&master_password.as_bytes());
        let encoded_data = cocoon.parse(&mut file)?;
                
        let vault = Vault {
            master_pwd: "".to_owned(),
            account:  Account::try_from_slice(&encoded_data).unwrap(),
            file_path: path.to_owned(),
        };

        Ok(vault)
    }
    
}

impl Drop for Vault {
    fn drop(&mut self) {
        let encoded_account =self.account.try_to_vec().unwrap();

        let mut file = File::create(file_path).expect("Could not create db file.");
        
        let cocoon = Cocoon::new(&self.master_pwd.as_bytes());
        // Dump the serialized database into a file as an encrypted container.
        cocoon.dump(encoded_account, &mut file).expect("Could not dump encrpyted data into db file.");

        // make file only readable
        let metadata = file.metadata().expect("Could not obtain file attributes");
        let mut permissions = metadata.permissions();
        permissions.set_readonly(true);
    }
}
