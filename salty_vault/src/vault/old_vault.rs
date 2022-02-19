use super::account::Account;
use borsh::{BorshDeserialize, BorshSerialize};
use cocoon::Cocoon;
use dirs;

use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::string::String;

#[cfg(test)]
mod vault_test {
    use super::Vault;
    use crate::utils::workspace::Workspace;
    use crate::vault::account::{Account, AccountEntry};
    use std::panic;
    #[test]
    fn test_create_vault() {
        let mut ws = Workspace::new();
        let _result = panic::catch_unwind(move || {
            // TODO: remove once fs::try_exist is merged in rust stable
            ws.setup_workspace();
        });

        let v = Vault::create("vaultname", "masterPassword", "salt123545")
            .expect("Failed creating vault!");

        assert_eq!(v.salt, "salt123545");
        assert_eq!(v.master_pwd, "masterPassword");
        let home_dir = dirs::home_dir().unwrap().to_str().unwrap().to_owned();
        assert_eq!(v.file_path, home_dir + "/.salty/vaultname.slt");
    }

    #[test]
    fn test_store_in_disk_then_load_vault_from_disk() {
        let mut ws = Workspace::new();
        let _result = panic::catch_unwind(move || {
            ws.setup_workspace();
        });

        {
            let mut v = Vault::create("vaultname", "masterPassword", "salt123545")
                .expect("Failed creating vault!");

            let account = &mut v.account;
            account
                .add(AccountEntry::new("google", "andrew", "123456789"))
                .unwrap();
            account
                .add(AccountEntry::new("amazon", "andrew", "123456789"))
                .unwrap();
            account
                .add(AccountEntry::new("facebook", "andrew", "123456789"))
                .unwrap();

            assert_eq!(v.account.size(), 3);
            v.save_to_disk();
        }
        {
            let loaded_v =
                Vault::load("masterPassword", "vaultname").expect("Failed loading vault!");
            assert_eq!(loaded_v.salt, "salt123545");
            assert_eq!(loaded_v.master_pwd, "masterPassword");
            let home_dir = dirs::home_dir().unwrap().to_str().unwrap().to_owned();
            assert_eq!(loaded_v.file_path, home_dir + "/.salty/vaultname.slt");
            assert_eq!(loaded_v.account.size(), 3);
        }
    }
}

fn file_path(name: &str) -> String {
    dirs::home_dir()
        .expect("No home directory found in your system!")
        .join(".salty/")
        .join(name)
        .with_extension("slt")
        .to_str()
        .expect("invalid path to config directroy!")
        .to_owned()
}
#[derive(Debug)]
pub struct Vault {
    master_pwd: String,
    salt: String,
    account: Account,
    file_path: String,
}

#[derive(Debug)]
pub enum VaultError {
    IoError(io::Error),
    EncryptingError(cocoon::Error),
    InvalidVault,
}

impl From<io::Error> for VaultError {
    fn from(error: io::Error) -> Self {
        VaultError::IoError(error)
    }
}

impl From<cocoon::Error> for VaultError {
    fn from(error: cocoon::Error) -> Self {
        VaultError::EncryptingError(error)
    }
}

impl Vault {
    pub fn create<'a, 'b, 'c>(name: &'a str, master_pwd: &'b str, salt: &'c str) -> Option<Self> {
        Some(Vault {
            master_pwd: master_pwd.to_owned(),
            salt: salt.to_owned(),
            account: Account::new(),
            file_path: file_path(name),
        })
    }

    pub fn load<'a, 'b>(master_password: &'a str, name: &'b str) -> Result<Self, VaultError> {
        let vault_path = file_path(name);
        let mut path = PathBuf::from(&vault_path);
        path.is_file().then(|| 0).ok_or(VaultError::InvalidVault)?;

        let mut file = File::open(&path)?;
        let metadata = file.metadata()?;
        let mut permissions = metadata.permissions();
        permissions.set_readonly(false);

        let cocoon = Cocoon::new(&master_password.as_bytes());
        let encoded_data = cocoon.parse(&mut file)?;

        path.set_extension("slt.salt");
        let mut salt_file = File::open(path)?;
        let metadata = salt_file.metadata()?;
        let mut permissions = metadata.permissions();
        permissions.set_readonly(false);

        let cocoon = Cocoon::new(&master_password.as_bytes());
        let salt_encoded = cocoon.parse(&mut salt_file)?;

        let vault = Vault {
            master_pwd: master_password.to_owned(),
            salt: String::from_utf8(salt_encoded).unwrap(),
            account: Account::try_from_slice(&encoded_data).unwrap(),
            file_path: file_path(name),
        };

        Ok(vault)
    }

    pub fn save_to_disk(&self) {
        //
        // Save salt data in disk
        //
        let salt_file_path = self.file_path.clone() + ".salt";
        let mut salt_file = File::create(&salt_file_path).expect("Could not create db file.");
        let cocoon = Cocoon::new(&self.master_pwd.as_bytes());
        // Dump data
        cocoon
            .dump(self.salt.as_bytes().to_vec(), &mut salt_file)
            .expect("Could not dump encrpyted data into db file.");

        //
        // Save vault data in disk
        //
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
