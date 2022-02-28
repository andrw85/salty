pub use clap::Parser;

#[derive(Parser, Default)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// Sets the port number to listen on.
    #[clap(long, default_value_t = 50051)]
    pub port: u16,
    /// Sets the shutdown duration timeout in milliseconds.
    #[clap(long, default_value_t = 30)]
    pub shutdown_timeout: u64,
}
