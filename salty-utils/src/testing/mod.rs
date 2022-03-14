use dirs;
use std::fs;

pub trait Testing {
    fn default() -> Self;
}

pub fn clean_salty_home_workspace() {
    let home_dir = dirs::home_dir()
        .expect("No home directory found in your system!")
        .join(".salty/");
    fs::remove_dir_all(&home_dir).expect("Couldn't remove workspace directory");
    fs::create_dir(&home_dir).expect("Couldn't create workspace directory");
}
