use super::StorageError;
use crate::utils::Cipherble;
use crate::vault::account::{Account, Vault};
use borsh::{BorshDeserialize, BorshSerialize};
use cocoon::Cocoon;
use dirs;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::result::Result;

// Storing  data from a vault into a file can be achived throughout the HardDiskStorage trait.

pub trait HardDiskStorage
where
    Self: Sized,
{
    fn store(&self) -> Result<(), StorageError>;
    fn load<'a, 'b>(
        &self,
        account_name: &'a str,
        account_password: &'b str,
    ) -> Result<Self, StorageError>;
}

impl From<cocoon::Error> for StorageError {
    fn from(_error: cocoon::Error) -> Self {
        StorageError::Cocoon
    }
}

impl From<io::Error> for StorageError {
    fn from(_error: io::Error) -> Self {
        StorageError::Cocoon
    }
}

impl<T> HardDiskStorage for Account<T>
where
    T: Cipherble,
    Self: BorshSerialize + BorshDeserialize,
{
    fn store(&self) -> Result<(), StorageError> {
        let file_path = file_path(self.name());
        //
        // Save vault data in disk
        //

        let encoded_account = self.try_to_vec().unwrap();
        let mut file = File::create(&file_path).expect("Could not create db file.");
        let cocoon = <T as Cipherble>::from_bytes(self.password());
        // Dump the serialized database into a file as an encrypted container.
        cocoon
            .dump(encoded_account, &mut file)
            .expect("Could not dump encrpyted data into db file.");

        // make file only readable
        let metadata = file.metadata().expect("Could not obtain file attributes");
        let mut permissions = metadata.permissions();
        permissions.set_readonly(true);
        Ok(())
    }

    fn load<'a, 'b>(
        &self,
        account_name: &'a str,
        account_password: &'b str,
    ) -> Result<Self, StorageError> {
        let vault_path = file_path(account_name);
        let path = PathBuf::from(&vault_path);
        path.is_file().then(|| 0).ok_or(StorageError::Cocoon)?;

        let mut file = File::open(&path)?;
        let metadata = file.metadata()?;
        let mut permissions = metadata.permissions();
        permissions.set_readonly(false);

        let cocoon = Cocoon::new(&account_password.as_bytes());
        let encoded_data = cocoon.parse(&mut file)?;

        Ok(Self::try_from_slice(&encoded_data).unwrap())
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
