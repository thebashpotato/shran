use crate::error::{ShranError, ShranErrorType};
use clap::{App, AppSettings, Arg, ArgMatches};
use std::path::Path;

#[derive(Debug)]
pub struct ActiveCommand {
    pub sub_command: String,
    pub argument: String,
}

impl ActiveCommand {
    pub fn new(sub_command: &str, argument: &str) -> Self {
        Self {
            sub_command: sub_command.to_string(),
            argument: argument.to_string(),
        }
    }
}

/// Wrapper around the clap command line interface library.
///
/// # Example
///
/// ```no_run
/// let cli = Cli::new().unwrap_or_else(|error: ShranError| {
///     eprintln!("{}", error);
///     std::process::exit(1);
/// });
/// ```
#[derive(Debug)]
pub struct Cli {
    active_command: ActiveCommand,
}

impl<'e> Cli {
    pub fn new() -> ShranErrorType<'e, Self> {
        let m: ArgMatches = App::new("shran")
            .author("Matt Williams matt.k.williams@protonmail.com")
            .version("0.1.0")
            .about("A command line tool for automating the process of building and deploying a Bitcoin node")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(
                App::new("generate")
                    .setting(AppSettings::ArgRequiredElseHelp)
                    .about("Generate a build configuration for a specified proof of work blockchain")
                    .short_flag('G')
                    .arg(
                        Arg::new("bitcoin")
                            .long("btc")
                            .help("Generate a build.yaml configuration for the Bitcoin source code")
                            .conflicts_with_all(&["litecoin"])
                            .takes_value(false)
                    )
                    .arg(
                        Arg::new("litecoin")
                            .long("ltc")
                            .help("Generate a build.yaml configuration for the Litecoin source code")
                            .takes_value(false)
                    )
                )
            .subcommand(
                App::new("build")
                    .setting(AppSettings::ArgRequiredElseHelp)
                    .about("Execute a compilation strategy")
                    .short_flag('B')
                    .arg(
                        Arg::new("strategy")
                            .short('s')
                            .long("strategy")
                            .help("Path to a custom build.yaml strategy")
                            .takes_value(true)
                    )
                )
            .subcommand(
                App::new("auth")
                    .setting(AppSettings::ArgRequiredElseHelp)
                    .about("Authorize shran access to a github via the api")
                    .short_flag('A')
                    .arg(
                        Arg::new("token")
                            .long("with-token")
                            .help("The github token")
                            .takes_value(true)
                    )
                )
            .get_matches();

        let active_command: ActiveCommand = Self::get_active_command(&m)?;

        Ok(Self { active_command })
    }

    pub fn active_command(&self) -> &ActiveCommand {
        &self.active_command
    }

    fn get_active_command(matches: &ArgMatches) -> ShranErrorType<'e, ActiveCommand> {
        match matches.subcommand() {
            Some(("generate", generate_matches)) => {
                if generate_matches.is_present("bitcoin") {
                    Ok(ActiveCommand::new("generate", "bitcoin"))
                } else {
                    Ok(ActiveCommand::new("generate", "litecoin"))
                }
            }
            Some(("build", build_matches)) => {
                let arg = build_matches.value_of("strategy").unwrap();
                if !Path::new(&arg).exists() {
                    return Err(ShranError::BuildFileError {
                        msg: arg.to_string(),
                        file: file!(),
                        line: line!(),
                    });
                }
                Ok(ActiveCommand::new("build", arg))
            }
            Some(("auth", auth_matches)) => {
                let arg = auth_matches.value_of("token").unwrap();
                Ok(ActiveCommand::new("auth", arg))
            }
            _ => unreachable!(),
        }
    }
}
