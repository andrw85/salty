use super::StorageError;
use crate::{
    logs::debug,
    vault::{Account, Vault},
};
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
    fn exists(&self) -> bool;
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
        logs::debug!("Starting process for storing account in disk...");
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

        logs::debug!("Wrote salt to disk...");

        // Dump the serialized database into a file as an encrypted container.
        let encoded_account = self.try_to_vec().unwrap();
        cocoon
            .dump(encoded_account, &mut file)
            .expect("Could not dump encrpyted data into db file.");

        permissions.set_readonly(true);

        logs::debug!("Wrote account to disk...");

        Ok(())
    }

    fn load_from_disk<'a, 'b>(&mut self) -> Result<(), StorageError> {
        debug!(format!(
            "Checking account {} exists in disk before loading...",
            self.name()
        ));
        let vault_path = file_path(self.name());
        let path = PathBuf::from(&vault_path);
        path.is_file()
            .then(|| 0)
            .ok_or(StorageError::NoAccountFile)?;

        logs::debug!("Loading salt from disk...");
        let salt_path = PathBuf::from(vault_path.to_string() + ".salt");
        salt_path
            .is_file()
            .then(|| 0)
            .ok_or(StorageError::NoSaltFile)?;
        let salt_bytes = fs::read(salt_path).ok().ok_or(StorageError::ReadingSalt)?;
        let salt: [u8; 32] = <[u8; 32]>::try_from(salt_bytes).unwrap();

        logs::debug!("Salt Loaded. Loadind account...");
        let mut file = File::open(&path)?;
        let cocoon = self.cipher().with_seed(&self.password(), salt);
        let encoded_data = cocoon.parse(&mut file).ok().ok_or(StorageError::Cocoon)?;

        *self = Self::try_from_slice(&encoded_data).unwrap();
        logs::debug!("Account loaded!");
        Ok(())
    }

    fn exists(&self) -> bool {
        let vault_path = file_path(self.name());
        let path = PathBuf::from(&vault_path);
        debug!(format!(
            "Checking account {} exists in disk ... returned {}",
            self.name(),
            path.is_file()
        ));
        path.is_file()
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
