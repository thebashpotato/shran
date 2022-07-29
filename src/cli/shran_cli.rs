use super::commands::{ActiveCommand, ArgName, Argument, SubCommandName};
use crate::error::ShranError;
use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, ArgMatches, Command};
use std::error::Error;
use std::fmt;
use std::path::Path;

/// Wrapper around the clap command line interface library.
///
/// # Example
/// ```no_run
/// match Cli::new() {
///     Ok(cli) => {
///         ...
///     },
///     Err(e) => eprintln!("{}", e);
/// }
/// ```
#[derive(Debug)]
pub struct Cli {
    active_command: ActiveCommand,
}

impl fmt::Display for Cli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.active_command)
    }
}

impl<'e> Cli {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let m: ArgMatches = Command::new(crate_name!())
            .author(crate_authors!())
            .version(crate_version!())
            .about(crate_description!())
            .subcommand_required(true)
            .subcommand(
                Command::new(SubCommandName::AUTH)
                    .arg_required_else_help(true)
                    .about("Authorize shran access to a github via the api")
                    .short_flag('A')
                    .arg(
                        Arg::new(ArgName::TOKEN)
                            .long("token")
                            .help("The github token")
                            .takes_value(true),
                    ),
            )
            .subcommand(
                Command::new(SubCommandName::BUILD)
                    .arg_required_else_help(true)
                    .about("Execute a compilation strategy")
                    .short_flag('B')
                    .arg(
                        Arg::new(ArgName::STRATEGY)
                            .long("strategy")
                            .help("Path to a custom build.yaml strategy")
                            .takes_value(true),
                    ),
            )
            .subcommand(
                Command::new(SubCommandName::FETCH)
                    .arg_required_else_help(true)
                    .about("List, download and manage bitcoin source code from github and on your local machine")
                    .short_flag('F')
                    .arg(
                        Arg::new(ArgName::LIST_REMOTE)
                            .long("list-remote")
                            .help("List all available versions available to download from bitcoins repository")
                            .takes_value(false)
                            .conflicts_with_all(&[ArgName::LATEST, ArgName::LIST_LOCAL, ArgName::TAG]),
                    )
                    .arg(
                        Arg::new(ArgName::LIST_LOCAL)
                            .long("list-local")
                            .help("List versions already installed on your system")
                            .takes_value(false)
                            .conflicts_with_all(&[ArgName::LATEST, ArgName::TAG]),
                    )
                    .arg(
                        Arg::new(ArgName::LATEST)
                            .long("latest")
                            .help("Fetch the latest version release from github")
                            .takes_value(false)
                            .conflicts_with_all(&[ArgName::TAG]),
                    )
                    .arg(
                        Arg::new(ArgName::TAG)
                            .long("tag")
                            .help("Download a version specified by tag")
                            .takes_value(true),
                    ),
            )
            .subcommand(
                Command::new(SubCommandName::GENERATE)
                    .arg_required_else_help(true)
                    .about(
                        "Generate a build configuration for a specified proof of work blockchain",
                    )
                    .short_flag('G')
                    .arg(
                        Arg::new(ArgName::BITCOIN)
                            .long("btc")
                            .help("Generate a build.yaml configuration for the Bitcoin source code")
                            .conflicts_with_all(&[ArgName::LITECOIN])
                            .takes_value(false),
                    )
                    .arg(
                        Arg::new(ArgName::LITECOIN)
                            .long("ltc")
                            .help(
                                "Generate a build.yaml configuration for the Litecoin source code",
                            )
                            .takes_value(false),
                            ),
            )
            .get_matches();
        let active_command: ActiveCommand = Self::get_active_command(&m)?;

        Ok(Self { active_command })
    }

    fn get_active_command(matches: &ArgMatches) -> Result<ActiveCommand, Box<dyn Error>> {
        match matches.subcommand() {
            Some((SubCommandName::AUTH, auth_matches)) => {
                let arg = auth_matches.value_of(ArgName::TOKEN).unwrap();
                Ok(ActiveCommand::new(
                    SubCommandName::AUTH,
                    Argument {
                        value: Some(String::from(arg)),
                        name: ArgName::TOKEN.to_string(),
                    },
                ))
            }
            Some((SubCommandName::BUILD, build_matches)) => {
                let arg = build_matches.value_of(ArgName::STRATEGY).unwrap();
                if !Path::new(&arg).exists() {
                    return Err(Box::new(ShranError::BuildFileError {
                        msg: arg.to_string(),
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    }));
                }
                Ok(ActiveCommand::new(
                    SubCommandName::BUILD,
                    Argument {
                        value: Some(String::from(arg)),
                        name: ArgName::STRATEGY.to_string(),
                    },
                ))
            }
            Some((SubCommandName::FETCH, fetch_matches)) => {
                let mut active_arg: Argument = Default::default();
                if fetch_matches.is_present(ArgName::LIST_REMOTE) {
                    active_arg.name = ArgName::LIST_REMOTE.to_string();
                } else if fetch_matches.is_present(ArgName::LIST_LOCAL) {
                    active_arg.name = ArgName::LIST_LOCAL.to_string();
                } else if fetch_matches.is_present(ArgName::LATEST) {
                    active_arg.name = ArgName::LATEST.to_string();
                } else {
                    let arg = fetch_matches.value_of(ArgName::TAG).unwrap();
                    active_arg.value = Some(String::from(arg));
                    active_arg.name = ArgName::TAG.to_string();
                }
                Ok(ActiveCommand::new(SubCommandName::FETCH, active_arg))
            }

            Some((SubCommandName::GENERATE, generate_matches)) => {
                let mut active_arg: Argument = Default::default();
                if generate_matches.is_present(ArgName::BITCOIN) {
                    active_arg.name = ArgName::BITCOIN.to_string();
                } else {
                    active_arg.name = ArgName::LITECOIN.to_string();
                }
                Ok(ActiveCommand::new(SubCommandName::GENERATE, active_arg))
            }
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    pub fn subcommand_auth(&self) -> bool {
        &self.active_command.sub_command() == &SubCommandName::AUTH
    }

    #[inline(always)]
    pub fn subcommand_build(&self) -> bool {
        &self.active_command.sub_command() == &SubCommandName::BUILD
    }

    #[inline(always)]
    pub fn subcommand_fetch(&self) -> bool {
        &self.active_command.sub_command() == &SubCommandName::FETCH
    }

    #[inline(always)]
    pub fn subcommand_generate(&self) -> bool {
        &self.active_command.sub_command() == &SubCommandName::GENERATE
    }

    #[inline(always)]
    pub fn args(&self) -> Argument {
        self.active_command.arg()
    }
}
