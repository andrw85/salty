use super::account::Account;
use borsh::{BorshDeserialize, BorshSerialize};
use cocoon::Cocoon;
use dirs;
use passwords::PasswordGenerator;
use std::fs;
use std::fs::File;
use std::path::Path;

fn default_file_path() -> String {
    dirs::home_dir()
        .expect("No home directory found in your system!")
        .join(".salty/")
        .join("vault.slt")
        .to_str()
        .expect("invalid path to config directroy!")
        .to_owned()
}

pub struct Vault {
    master_pwd: String,
    salt: String,
    pub account: Account,
    file_path: String,
}

impl Vault {
    pub fn new<'a, 'b>(master_pwd: &'a str, salt: &'b str) -> Self {
        let default_dir = dirs::home_dir()
            .expect("No home directory found in your system!")
            .join(".salty/")
            .to_str()
            .expect("invalid path to config directroy!")
            .to_owned();

        fs::create_dir(default_dir).ok();

        Vault {
            master_pwd: master_pwd.to_owned(),
            salt: salt.to_owned(),
            account: Account::new(),
            file_path: default_file_path(),
        }
    }

    pub fn default<'a>(master_pwd: &'a str) -> Result<Self, cocoon::Error> {
        Self::from_file(master_pwd, &default_file_path())
    }

    pub fn salt() -> String {
        let salt_file = format!("{}{}", default_file_path(), ".salt");
        fs::read_to_string(&salt_file).expect("Unable to read salt file")
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
            salt: Self::salt(),
            account: Account::try_from_slice(&encoded_data).unwrap(),
            file_path: path.to_owned(),
        };

        Ok(vault)
    }

    pub fn exists() -> bool {
        return Path::new(&default_file_path()).exists();
    }
}

impl Drop for Vault {
    fn drop(&mut self) {
        let encoded_account = self.account.try_to_vec().unwrap();

        let salt_file = self.file_path.clone() + ".salt";
        fs::write(&salt_file, self.salt.clone()).expect("Unable to write salt file");

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
