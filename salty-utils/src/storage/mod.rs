mod error;
pub use error::StorageError;
mod harddisk_storage;
pub use harddisk_storage::HardDiskStorage;

#[cfg(test)]
mod tests {
    use super::HardDiskStorage;
    use crate::security::Cipher;
    use crate::vault::{Account, AccountEntry, MasterPassPhrase};
    #[test]
    fn test_harddisk_storage() {
        let plain_pwd = "my_pass".to_string();
        let cipher = Cipher::Fast;
        let pass_phrase = MasterPassPhrase::new(cipher.clone(), &plain_pwd);
        let mut account = Account::default("my_account", pass_phrase);
        let entry = AccountEntry::new("google", "andrew", "123456789");

        assert!(account.add(entry).is_ok());
        assert_eq!(account.size(), 1);
        let _res = account.store_to_disk().expect("Failed storing account!");

        let account_loaded = Account::load_from_disk(cipher.clone(), "my_account", "my_pass")
            .expect("Failed loading account!");

        assert_eq!(account_loaded, account);
        assert_eq!(account_loaded.size(), account.size());
    }
}
