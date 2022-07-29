mod cli;
mod config;
mod error;
mod github;
mod strategies;

pub use cli::commands::{ActiveCommand, ArgName, SubCommandName};
pub use cli::Cli;
pub use config::{FileSystemManager, ShranDefault, ShranFile};
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

fn run_auth(token: &String) -> Result<(), Box<dyn std::error::Error>> {
    let fs = FileSystemManager::new()?;
    fs.write_token(token.to_owned())?;

    Ok(())
}

async fn run_get_latest() -> Result<GitRelease, Box<dyn std::error::Error>> {
    let fs = FileSystemManager::new()?;
    let token = fs.read_token()?;
    let gclient = GithubClient::new(token)?;
    let release: GitRelease = gclient.get_latest_release().await?;
    Ok(release)
}

async fn run_get_tagged_release(tag: String) -> Result<GitRelease, Box<dyn std::error::Error>> {
    let fs = FileSystemManager::new()?;
    let token = fs.read_token()?;
    let gclient = GithubClient::new(token)?;
    let release: GitRelease = gclient.get_tagged_release(&tag).await?;
    Ok(release)
}

async fn run_get_remote() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let fs = FileSystemManager::new()?;
    let token = fs.read_token()?;
    let gclient = GithubClient::new(token)?;
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
                if let Err(e) = run_auth(&cli.args().value.unwrap()) {
                    eprintln!("{}", e);
                    exit_code = ExitCode::FAILURE;
                }
            }

            if cli.subcommand_build() {
                run_build(&cli.args().value.unwrap());
            }

            if cli.subcommand_fetch() {
                match cli.args().value {
                    Some(tag) => match run_get_tagged_release(tag).await {
                        Ok(release) => {
                            println!("{}", release);
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                            exit_code = ExitCode::FAILURE;
                        }
                    },
                    None => {
                        if cli.args().name == ArgName::LIST_REMOTE {
                            match run_get_remote().await {
                                Ok(tags) => {
                                    for tag in tags {
                                        println!("{}", tag);
                                    }
                                }
                                Err(e) => {
                                    eprintln!("{}", e);
                                    exit_code = ExitCode::FAILURE;
                                }
                            }
                        }
                        if cli.args().name == ArgName::LATEST {
                            match run_get_latest().await {
                                Ok(release) => {
                                    println!("{}", release);
                                }
                                Err(e) => {
                                    eprintln!("{}", e);
                                    exit_code = ExitCode::FAILURE;
                                }
                            }
                        }
                        if cli.args().name == ArgName::LIST_LOCAL {
                            println!("{} not implemented yet", cli.args().name);
                        }
                    }
                }
            }

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
