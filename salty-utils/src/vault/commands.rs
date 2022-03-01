use serde::{Deserialize, Serialize};
use std::env;
pub use structopt::StructOpt;

/// Salty  is an open implementation of a password management system.
#[derive(Serialize, Deserialize, StructOpt, Debug)]
#[structopt(name = env!("CARGO_PKG_NAME"))]
#[structopt(version = env!("CARGO_PKG_VERSION"))]
#[structopt(about = env!("CARGO_PKG_DESCRIPTION"))]
#[structopt(author = env!("CARGO_PKG_AUTHORS"))]
pub enum Cmd {
    /// Create a vault
    Create { vault_name: String },
    /// Login to a vault
    Login { vault_name: String },
    /// Add an entry
    Add(AddOpt),
    /// Show entries
    Show,
}

#[derive(StructOpt, Debug, Serialize, Deserialize)]
pub struct AddOpt {
    #[structopt(short, long, required = true)]
    pub site: String,
    #[structopt(short, long, required = true)]
    pub user: String,
}
