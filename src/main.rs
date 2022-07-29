mod cli;
mod config;
mod error;
mod github;
mod strategies;

pub use cli::commands::{ActiveCommand, SubCommandName};
pub use cli::Cli;
pub use config::{ShranDefault, ShranFile};
pub use error::ShranError;
pub use github::{GitRelease, GithubClient};
use std::process::ExitCode;
pub use strategies::bitcoin::{BuildOptionName, BuildStrategy, OptionEnabled};

fn run_generate(node_type: &String) {
    println!("Generating build for: {}", node_type);
}

fn run_build(path: &String) {
    println!("Build file path {}", path);
}

fn run_auth(token: &String) {
    println!("Running auth with token: {}", token);
}

async fn run_get_latest(token: &String) -> Result<GitRelease, Box<dyn std::error::Error>> {
    let gclient = GithubClient::new(token.to_owned())?;
    let release: GitRelease = gclient.get_latest_release().await?;
    Ok(release)
}

async fn run_get_tagged_release(token: &String) -> Result<GitRelease, Box<dyn std::error::Error>> {
    let gclient = GithubClient::new(token.to_owned())?;
    let tag = String::from("v0.21.0");
    let release: GitRelease = gclient.get_tagged_release(&tag).await?;
    Ok(release)
}

async fn run_get_all_available_tags(
    token: &String,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let gclient = GithubClient::new(token.to_owned())?;
    let tags: Vec<String> = gclient.get_all_tags().await?;
    Ok(tags)
}

#[tokio::main]
async fn main() -> ExitCode {
    let mut exit_code = ExitCode::SUCCESS;
    match Cli::new() {
        Ok(cli) => {
            dbg!("{}", &cli);
            if cli.subcommand_auth() {
                run_auth(&cli.args().value.unwrap());
            }

            if cli.subcommand_build() {
                run_build(&cli.args().value.unwrap());
            }

            if cli.subcommand_fetch() {}

            if cli.subcommand_generate() {
                run_generate(&cli.args().name)
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            exit_code = ExitCode::FAILURE;
        }
    }
    exit_code
}
