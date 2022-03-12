pub use clap::Parser;
use salty_utils::security::Cipher;
#[cfg(test)]
pub use salty_utils::testing::Testing;

use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// Sets the port number to listen on.
    #[clap(long, default_value_t = 50051)]
    pub port: u16,
    /// Sets the shutdown duration timeout in seconds.
    #[clap(long, default_value_t = 30, validator = shutdown_timeout_range)]
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

fn shutdown_timeout_range(s: &str) -> Result<(), String> {
    const TIMEOUT_RANGE: RangeInclusive<u64> = 30..=300;
    u64::from_str(s)
        .map(|timeout| TIMEOUT_RANGE.contains(&timeout))
        .map_err(|e| e.to_string())
        .and_then(|result| match result {
            true => Ok(()),
            false => Err(format!(
                "Timeout should be in range {}-{} for security reasons",
                TIMEOUT_RANGE.start(),
                TIMEOUT_RANGE.end()
            )),
        })
}
