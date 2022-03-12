use std::error::Error;
use std::fmt;
#[derive(Debug)]
pub enum StorageError {
    Cocoon,
    NoAccountFile,
    NoSaltFile,
    ReadingSalt,
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "")
    }
}

impl Error for StorageError {}
