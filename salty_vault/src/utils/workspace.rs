use std::fs;
use std::sync::Mutex;

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

    pub fn setup_workspace(&mut self) {
        let default_dir = self.mutex.lock().unwrap();
        fs::create_dir(default_dir.as_path()).expect("Could not create salty home folder!");
    }

    pub fn delete_workspace(&mut self) {
        let default_dir = self.mutex.lock().unwrap();
        fs::remove_dir_all(default_dir.as_path()).expect("Could not remove salty home folder!");
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
