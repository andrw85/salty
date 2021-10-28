use super::account::Account;
use borsh::{BorshDeserialize, BorshSerialize};
use cocoon::Cocoon;
use std::fs::File;
use std::path::Path;

static DEFAULT_FILE_PATH: &str = "vault.slt";

pub struct Vault {
    master_pwd: String,
    pub account: Account,
    file_path: String,
}

impl Vault {
    pub fn new<'a>(master_pwd: &'a str) -> Self {
        Vault {
            master_pwd: master_pwd.to_owned(),
            account: Account::new(),
            file_path: DEFAULT_FILE_PATH.to_owned(),
        }
    }

    pub fn default<'a>(master_pwd: &'a str) -> Result<Self, cocoon::Error> {
        Self::from_file(master_pwd, DEFAULT_FILE_PATH)
    }

    pub fn from_file<'a, 'b>(
        master_password: &'a str,
        path: &'b str,
    ) -> Result<Self, cocoon::Error> {
        let mut file = File::open(&path)?;
        let metadata = file.metadata()?;
        let mut permissions = metadata.permissions();
        permissions.set_readonly(false);

        let cocoon = Cocoon::new(&master_password.as_bytes());
        let encoded_data = cocoon.parse(&mut file)?;

        let vault = Vault {
            master_pwd: master_password.to_owned(),
            account: Account::try_from_slice(&encoded_data).unwrap(),
            file_path: path.to_owned(),
        };

        Ok(vault)
    }

    pub fn exists() -> bool {
        return Path::new(DEFAULT_FILE_PATH).exists();
    }
}

impl Drop for Vault {
    fn drop(&mut self) {
        let encoded_account = self.account.try_to_vec().unwrap();

        let mut file = File::create(&self.file_path).expect("Could not create db file.");

        let cocoon = Cocoon::new(&self.master_pwd.as_bytes());
        // Dump the serialized database into a file as an encrypted container.
        cocoon
            .dump(encoded_account, &mut file)
            .expect("Could not dump encrpyted data into db file.");

        // make file only readable
        let metadata = file.metadata().expect("Could not obtain file attributes");
        let mut permissions = metadata.permissions();
        permissions.set_readonly(true);
    }
}
