mod error;
pub use error::StorageError;
mod harddisk_storage;
pub use harddisk_storage::HardDiskStorage;

#[cfg(test)]
mod tests {
    use super::HardDiskStorage;
    use crate::vault::account::Account;

    #[test]
    fn test_harddisk_storage() {
        let account = Account::create_with_fast_cipher("my_account", "my_pass");
        let res = account.store();
        assert_eq!(res.is_ok(), true);
    }
}
