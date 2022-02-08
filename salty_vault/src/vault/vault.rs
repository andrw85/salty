use super::account::Account;
use borsh::{BorshDeserialize, BorshSerialize};
use cocoon::Cocoon;
use dirs;

use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;
use std::string::String;

#[cfg(test)]
mod vault_test {
    use super::Vault;
    use crate::utils::workspace;
    use std::panic;

    #[test]
    fn test_create_vault() {
        let _result = panic::catch_unwind(|| {
            workspace::delete_workspace();
        });
        workspace::setup_workspace();
        let v = Vault::create("vaultname", "masterPassword", "salt123545")
            .expect("Failed creating vault!");
        assert_eq!(v.salt, "salt123545");
        assert_eq!(v.master_pwd, "masterPassword");
        assert_eq!(v.file_path, "/home/salty/.salty/vaultname.slt");
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
        let path = Path::new(&vault_path);
        path.is_file().then(|| 0).ok_or(VaultError::InvalidVault)?;

        let mut file = File::open(path)?;
        let metadata = file.metadata()?;
        let mut permissions = metadata.permissions();
        permissions.set_readonly(false);

        let cocoon = Cocoon::new(&master_password.as_bytes());
        let encoded_data = cocoon.parse(&mut file)?;

        let salt_path = path.join(".salt");
        let mut salt_file = File::open(salt_path)?;
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

    pub fn make_persistent(&mut self) {
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
