use crate::security::generate_salt;
use std::fs;
use std::sync::Mutex;

#[derive(Debug)]
pub enum Error {
    FailedCreatingDirectory,
    FailedDeletingDirectory,
    FailedDeletingAccountFile,
    FailedDeletingAccountSaltFile,
}

type Result<T> = std::result::Result<T, Error>;

pub struct Workspace {
    mutex: Mutex<std::path::PathBuf>,
}

impl Workspace {
    pub fn new() -> Self {
        let path = dirs::home_dir()
            .expect("No home directory found in your system!")
            .join(".salty/");

        Workspace {
            mutex: Mutex::new(path),
        }
    }

    pub fn setup_workspace(&mut self) -> Result<()> {
        let default_dir = self.mutex.lock().unwrap();
        match fs::create_dir(default_dir.as_path()) {
            Ok(_) => return Ok(()),
            Err(_) => return Err(Error::FailedCreatingDirectory),
        }
    }

    pub fn delete_workspace(&mut self) -> Result<()> {
        let default_dir = self.mutex.lock().unwrap();
        match fs::remove_dir_all(default_dir.as_path()) {
            Ok(_) => return Ok(()),
            Err(_) => return Err(Error::FailedDeletingDirectory),
        }
    }
    pub fn reset_workspace(&mut self) -> Result<()> {
        generate_salt();
        let _ = self.delete_workspace();
        self.setup_workspace()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_setting_vault_workspace() {
//         let file_path = dirs::home_dir().expect("No home directory").join(".salty/");
//         let mut space = Workspace::new();

//         println!("checking if path ");
//         if file_path.is_dir() {
//             println!("file path exists");
//             space.delete_workspace();
//         }

//         space.setup_workspace();
//     }
//     #[test]
//     #[should_panic]
//     fn test_deleting_vault_workspace() {
//         let file_path = dirs::home_dir().expect("No home directory").join(".salty/");
//         let mut space = Workspace::new();

//         println!("checking if path 2222 ");
//         if file_path.is_dir() {
//             println!("file path exists 2222");
//             space.delete_workspace();
//         }

//         space.delete_workspace();
//     }
// }
