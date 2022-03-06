mod cli;
mod config;
mod error;
mod strategy;

pub use cli::Cli;
pub use config::{ShranDefault, ShranFile};
pub use error::ShranError;
pub use strategy::bitcoin::BuildStrategy;

fn main() {
    let cli = Cli::new().unwrap_or_else(|error: ShranError| {
        eprintln!("{}", error);
        std::process::exit(1);
    });

    let ac = cli.active_command();

    println!("Subcommand: {}", ac.sub_command());
    println!("Argument: {}", ac.arg());

    let _ = strategy::bitcoin::BuildStrategy::new();
}
