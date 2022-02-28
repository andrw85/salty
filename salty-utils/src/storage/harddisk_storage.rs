use super::StorageError;
use crate::vault::{Account, Vault};
use borsh::{BorshDeserialize, BorshSerialize};
use dirs;
use std::convert::TryFrom;
use std::fs;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::result::Result;

// Storing  data from a vault into a file can be achived throughout the HardDiskStorage trait.

pub trait HardDiskStorage
where
    Self: Sized,
{
    fn store_to_disk(&self) -> Result<(), StorageError>;
    fn load_from_disk<'a, 'b>(&mut self) -> Result<(), StorageError>;
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

impl HardDiskStorage for Account
where
    Self: BorshSerialize + BorshDeserialize + Vault,
{
    fn store_to_disk(&self) -> Result<(), StorageError> {
        let file_path = file_path(self.name());

        //
        // Save vault data in disk
        //
        let mut file = File::create(&file_path).expect("Could not create db file.");

        // make file only readable
        let metadata = file.metadata().expect("Could not obtain file attributes");
        let mut permissions = metadata.permissions();
        permissions.set_readonly(false);

        let salt: [u8; 32] = self.salt().clone();
        let cocoon = self.cipher().with_seed(&self.password(), salt);
        // save the seed in disk:
        fs::write(file_path.to_string() + ".salt", self.salt())?;

        // Dump the serialized database into a file as an encrypted container.
        let encoded_account = self.try_to_vec().unwrap();
        cocoon
            .dump(encoded_account, &mut file)
            .expect("Could not dump encrpyted data into db file.");

        permissions.set_readonly(true);
        Ok(())
    }

    fn load_from_disk<'a, 'b>(&mut self) -> Result<(), StorageError> {
        let vault_path = file_path(self.name());
        let path = PathBuf::from(&vault_path);
        path.is_file().then(|| 0).ok_or(StorageError::Cocoon)?;

        let salt_bytes = fs::read(vault_path.to_string() + ".salt").expect("Failed to read salt!");
        let salt: [u8; 32] = <[u8; 32]>::try_from(salt_bytes).unwrap();

        let mut file = File::open(&path)?;
        let cocoon = self.cipher().with_seed(&self.password(), salt);
        let encoded_data = cocoon.parse(&mut file)?;

        *self = Self::try_from_slice(&encoded_data).unwrap();
        Ok(())
    }
}

pub fn file_path(name: &str) -> String {
    dirs::home_dir()
        .expect("No home directory found in your system!")
        .join(".salty/")
        .join(name)
        .with_extension("slt")
        .to_str()
        .expect("invalid path to config directroy!")
        .to_owned()
}
