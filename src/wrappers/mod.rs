pub mod hasher;
pub mod password;

pub use hasher::hash;
pub use password::{get_password, ask_for_new_password};