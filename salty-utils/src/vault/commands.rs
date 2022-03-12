use crate::storage;
pub use clap::Parser;
pub use serde::{Deserialize, Serialize};
use std::env;

/// Salty  is an open implementation of a password management system.
#[derive(Serialize, Deserialize, Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub enum Cmd {
    /// Create a vault
    Create(CreateCmd),
    /// Login to a vault
    Login(LoginCmd),
    /// Add an entry
    Add(AddCmd),
    /// Show entries
    Show(ShowCmd),
}

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct CreateCmd {
    #[structopt(short, long, required = true)]
    pub vault_name: String,
    #[structopt(short, long, required = true)]
    pub password: String,
}

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct LoginCmd {
    #[structopt(short, long, required = true)]
    pub vault_name: String,
    #[structopt(short, long, required = true)]
    pub password: String,
}

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct AddCmd {
    #[structopt(short, long, required = true)]
    pub site: String,
    #[structopt(short, long, required = true)]
    pub user: String,
}

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct ShowCmd {}

#[derive(Debug, Serialize, Deserialize)]
pub enum CmdErrorCode {
    Ok,
    StorageBackendError,
    AccountAlreadyExists,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CmdResponse {
    pub result: CmdErrorCode,
    pub message: String, // used only when error code is not CmdErrorCode::Ok
}
