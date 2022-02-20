mod error;
pub use error::StorageError;
mod harddisk_storage;
pub use harddisk_storage::HardDiskStorage;

#[cfg(test)]
mod tests {
    use super::HardDiskStorage;
    use crate::vault::{Account, AccountEntry};
    #[test]
    fn test_harddisk_storage() {
        let mut account = Account::create_with_fast_cipher("my_account", "my_pass");
        let entry = AccountEntry::new("google", "andrew", "123456789");

        assert!(account.add(entry).is_ok());
        assert_eq!(account.size(), 1);
        let _res = account.store_to_disk().expect("Failed storing account!");

        let mut account_loaded = Account::create_with_fast_cipher("my_account", "my_pass");
        account_loaded
            .load_from_disk()
            .expect("Failed loading account!");

        assert_eq!(account_loaded, account);
        assert_eq!(account_loaded.size(), account.size());
    }
}
