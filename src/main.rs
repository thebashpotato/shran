mod cli;
mod config;
mod error;
mod strategies;

pub use cli::commands::{ActiveCommand, SubCommandName};
pub use cli::Cli;
pub use config::{ShranDefault, ShranFile};
use curl::easy::Easy;
pub use error::ShranError;
use octocrab::models::repos::Release;
use octocrab::Octocrab;
use std::fs::File;
use std::io::prelude::*;
pub use strategies::bitcoin::{BuildOptionName, BuildStrategy, OptionEnabled};

const BITCOIN_BASE_URL: &str = "https://github.com/bitcoin/bitcoin/archive/refs/tags";
const FILE_EXTENSION: &str = ".tar.gz";

fn run_generate(node_type: &String) {
    println!("Generating build for: {}", node_type);
}

fn run_build(path: &String) {
    println!("Build file path {}", path);
}

fn download_release(url: &String) -> std::io::Result<bool> {
    let mut dst: Vec<u8> = Vec::new();
    let mut easy: Easy = Easy::new();
    easy.url(url)?;
    let _redirect = easy.follow_location(true);

    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                dst.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }
    {
        let mut file = File::create("bitcoin-v23-0.tar.gz")?;
        file.write_all(dst.as_slice())?;
    }
    Ok(true)
}

async fn run_auth(token: &String) -> octocrab::Result<String, Box<dyn std::error::Error>> {
    println!("Fetching latest release");

    // write some test octocrab code
    let octo = Octocrab::builder()
        .personal_token(token.to_owned())
        .build()?;

    let release: Release = octo
        .repos("bitcoin", "bitcoin")
        .releases()
        .get_latest()
        .await?;

    Ok(format!(
        "{}/{}{}",
        BITCOIN_BASE_URL, release.tag_name, FILE_EXTENSION
    ))
}

#[tokio::main]
async fn main() -> octocrab::Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::new().expect("Could not build Cli struct");
    let ac: &ActiveCommand = cli.active_command();

    if ac.sub_command() == SubCommandName::GENERATE {
        run_generate(ac.arg());
    }

    if ac.sub_command() == SubCommandName::BUILD {
        run_build(ac.arg());
    }

    if ac.sub_command() == SubCommandName::AUTH {
        let url = run_auth(ac.arg()).await?;
        if let Ok(success) = download_release(&url) {
            if success {
                println!("Download was a success");
            }
        }
    }
    Ok(())
}
