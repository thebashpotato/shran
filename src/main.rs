mod cli;
mod config;
mod error;
mod strategies;

pub use cli::commands::{ActiveCommand, SubCommandName};
pub use cli::Cli;
pub use config::{ShranDefault, ShranFile};
pub use error::ShranError;
use octocrab::models::repos::Release;
use octocrab::Octocrab;
pub use strategies::bitcoin::{BuildOptionName, BuildStrategy, OptionEnabled};

fn run_generate(node_type: &String) {
    println!("Generating build for: {}", node_type);
}

fn run_build(path: &String) {
    println!("Build file path {}", path);
}

async fn run_auth(token: &String) -> octocrab::Result<()> {
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

    println!("Release Author: {}", release.author.login);
    println!("Tag: {}", release.tag_name);
    println!("Assets Url: {}", release.assets_url);

    if let Some(tar_url) = release.tarball_url {
        println!("Download url: {}", tar_url);
        println!("Scheme End: {}", tar_url.path());
    }

    Ok(())
}

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    match Cli::new() {
        Ok(cli) => {
            let ac: &ActiveCommand = cli.active_command();

            if ac.sub_command() == SubCommandName::GENERATE {
                run_generate(ac.arg());
            }

            if ac.sub_command() == SubCommandName::BUILD {
                run_build(ac.arg());
            }

            if ac.sub_command() == SubCommandName::AUTH {
                run_auth(ac.arg()).await?;
            }
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
    Ok(())
}
