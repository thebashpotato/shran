mod cli;
mod config;
mod error;

pub use cli::commands::{ActiveCommand, SubCommandName};
pub use cli::Cli;
pub use config::{ShranDefault, ShranFile};
pub use error::ShranError;

fn run_generate(node_type: &String) {
    println!("Generating build for: {}", node_type);
}

fn run_build(path: &String) {
    println!("Build file path {}", path);
}

fn run_auth(token: &String) {
    println!("Running auth code {}", token);
}

fn main() {
    let cli = Cli::new().unwrap_or_else(|error: ShranError| {
        eprintln!("{}", error);
        std::process::exit(1);
    });

    let ac: &ActiveCommand = cli.active_command();

    if ac.sub_command() == SubCommandName::GENERATE {
        run_generate(ac.arg());
    }

    if ac.sub_command() == SubCommandName::BUILD {
        run_build(ac.arg());
    }

    if ac.sub_command() == SubCommandName::AUTH {
        run_auth(ac.arg());
    }
}
