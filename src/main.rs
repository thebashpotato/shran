mod cli;
mod config;
mod error;

pub use cli::Cli;
pub use config::{ShranDefault, ShranFile};
pub use error::ShranError;

fn main() {
    let cli = Cli::new().unwrap_or_else(|error: ShranError| {
        eprintln!("{}", error);
        std::process::exit(1);
    });

    println!("Subcommand: {}", cli.active_command().sub_command);
    println!("Argument: {}", cli.active_command().argument);
}
