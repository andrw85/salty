pub use clap::{Parser, Subcommand};
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
    #[clap(short, long, required = true)]
    pub vault_name: String,
    #[clap(short, long, required = true)]
    pub password: String,
    #[clap(short, long)]
    pub local: bool,
}

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct LoginCmd {
    #[clap(short, long, required = true)]
    pub vault_name: String,
    #[clap(short, long, required = true)]
    pub password: String,
}

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct AddCmd {
    #[clap(short, long, required = true)]
    pub site: String,
    #[clap(short, long, required = true)]
    pub user: String,
}

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct ShowCmd {}

#[derive(Debug, Serialize, Deserialize)]
pub enum CmdErrorCode {
    Ok,
    StorageBackendError,
    AccountAlreadyExists,
    AccountDoesNotExist,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CmdResponse {
    pub result: CmdErrorCode,
    pub message: String, // used only when error code is not CmdErrorCode::Ok
}
