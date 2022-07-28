mod cli;
mod config;
mod error;
mod github;
mod strategies;

pub use cli::commands::{ActiveCommand, SubCommandName};
pub use cli::Cli;
pub use config::{ShranDefault, ShranFile};
pub use error::ShranError;
pub use github::releases::{GitRelease, GithubClient};
pub use strategies::bitcoin::{BuildOptionName, BuildStrategy, OptionEnabled};

fn run_generate(node_type: &String) {
    println!("Generating build for: {}", node_type);
}

fn run_build(path: &String) {
    println!("Build file path {}", path);
}

//async fn run_get_latest(token: String) -> Result<GitRelease, Box<dyn std::error::Error>> {
//let gclient = GithubClient::new(token)?;
//let release: GitRelease = gclient.get_latest_release().await?;
//Ok(release)
//}

async fn run_get_all_available_tags(token: String) -> Result<(), Box<dyn std::error::Error>> {
    let gclient = GithubClient::new(token)?;
    let tags: Vec<String> = gclient.get_all_tags().await?;
    for tag in tags {
        println!("{}", tag);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> octocrab::Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::new().expect("Cli constructor failure");
    let ac: &ActiveCommand = cli.active_command();

    if ac.sub_command() == SubCommandName::GENERATE {
        run_generate(ac.arg());
    }

    if ac.sub_command() == SubCommandName::BUILD {
        run_build(ac.arg());
    }

    if ac.sub_command() == SubCommandName::AUTH {
        run_get_all_available_tags(ac.arg().to_owned()).await?
        //let release: GitRelease = run_auth(ac.arg().to_owned()).await?;
        //println!("Author: {}", release.author);
        //println!("Tag: {}", release.tag_name);
        //println!("Release branch: {}", release.release_branch);
    }
    Ok(())
}
