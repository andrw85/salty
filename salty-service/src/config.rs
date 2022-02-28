pub use clap::Parser;
use salty_utils::security::Cipher;
#[cfg(test)]
pub use salty_utils::testing::Testing;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// Sets the port number to listen on.
    #[clap(long, default_value_t = 50051)]
    pub port: u16,
    /// Sets the shutdown duration timeout in milliseconds.
    #[clap(long, default_value_t = 30)]
    pub shutdown_timeout: u64,
    // type of cipher used by default
    #[clap(skip = Cipher::Slow)]
    pub cipher: Cipher,
}

#[cfg(test)]
impl Testing for Config {
    fn default() -> Self {
        Config {
            port: 0,
            shutdown_timeout: 50u64,
            cipher: Cipher::Fast,
        }
    }
}
