use std::fs;

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::panic;
    #[test]
    fn test_setting_vault_workspace() {
        let result = panic::catch_unwind(|| {
            delete_workspace();
        });
        if result.is_ok() {
            setup_workspace();
        }
    }
    #[test]
    #[should_panic(expected = "Could not remove salty home folder!")]
    fn test_deleting_vault_workspace() {
        delete_workspace();
        delete_workspace();
    }

    #[test]
    #[should_panic(expected = "Could not create salty home folder!")]
    #[serial]
    fn test_vault_workspace_already_exists() {
        let _result = panic::catch_unwind(|| {
            delete_workspace();
        });
        setup_workspace();
        setup_workspace();
    }
}

pub fn setup_workspace() {
    let default_dir = dirs::home_dir()
        .expect("No home directory found in your system!")
        .join(".salty/")
        .to_str()
        .expect("invalid path to config directroy!")
        .to_owned();

    fs::create_dir(default_dir).expect("Could not create salty home folder!");
}

pub fn delete_workspace() {
    let default_dir = dirs::home_dir()
        .expect("No home directory found in your system!")
        .join(".salty/")
        .to_str()
        .expect("invalid path to config directroy!")
        .to_owned();
    fs::remove_dir_all(default_dir).expect("Could not remove salty home folder!");
}
