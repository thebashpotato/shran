mod cli;
mod error;

pub use cli::Cli;
pub use error::ShranError;

fn main() {
    let cli = Cli::new().unwrap_or_else(|error: ShranError| {
        eprintln!("{}", error);
        std::process::exit(1);
    });
    match cli.build_file() {
        Some(file) => println!("Build file: {}", file),
        None => println!("No build file was passed")
    }
}
