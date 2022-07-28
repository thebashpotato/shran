use super::commands::{ActiveCommand, ArgName, SubCommandName};
use crate::error::ShranError;
use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, ArgMatches, Command};
use std::error::Error;
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

impl<'e> Cli {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let m: ArgMatches = Command::new(crate_name!())
            .author(crate_authors!())
            .version(crate_version!())
            .about(crate_description!())
            .subcommand_required(true)
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
            .subcommand(
                Command::new(SubCommandName::BUILD)
                    .arg_required_else_help(true)
                    .about("Execute a compilation strategy")
                    .short_flag('B')
                    .arg(
                        Arg::new(ArgName::STRATEGY)
                            .short('s')
                            .long("build-strategy")
                            .help("Path to a custom build.yaml strategy")
                            .takes_value(true),
                    ),
            )
            .subcommand(
                Command::new(SubCommandName::AUTH)
                    .arg_required_else_help(true)
                    .about("Authorize shran access to a github via the api")
                    .short_flag('A')
                    .arg(
                        Arg::new(ArgName::TOKEN)
                            .long("with-token")
                            .help("The github token")
                            .takes_value(true),
                    ),
            )
            .get_matches();

        let active_command: ActiveCommand = Self::get_active_command(&m)?;

        Ok(Self { active_command })
    }

    pub fn active_command(&self) -> &ActiveCommand {
        &self.active_command
    }

    fn get_active_command(matches: &ArgMatches) -> Result<ActiveCommand, Box<dyn Error>> {
        match matches.subcommand() {
            Some((SubCommandName::GENERATE, generate_matches)) => {
                let active_arg: &str;
                if generate_matches.is_present(ArgName::BITCOIN) {
                    active_arg = ArgName::BITCOIN;
                } else {
                    active_arg = ArgName::LITECOIN;
                }
                Ok(ActiveCommand::new(SubCommandName::GENERATE, active_arg))
            }
            Some((SubCommandName::BUILD, build_matches)) => {
                let arg = build_matches.value_of(ArgName::STRATEGY).unwrap();
                if !Path::new(&arg).exists() {
                    return Err(Box::new(ShranError::BuildFileError {
                        msg: arg.to_string(),
                        file: file!(),
                        line: line!(),
                    }));
                }
                Ok(ActiveCommand::new(SubCommandName::BUILD, arg))
            }
            Some((SubCommandName::AUTH, auth_matches)) => {
                let arg = auth_matches.value_of(ArgName::TOKEN).unwrap();
                Ok(ActiveCommand::new(SubCommandName::AUTH, arg))
            }
            _ => unreachable!(),
        }
    }
}
