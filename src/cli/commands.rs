use std::default::Default;
use std::fmt;

/// All accepted subcommands that the shran cli accepts are
///
/// # Details
///
/// * auth
///     - currently only supports a github token
///
/// * build
///     - requires a path to a relavant build template
///
/// * fetch
///     - supports listing all bitcoin versions, download the latest version, and downloading a
///       specified version
///
/// * generate
///     - create a build template which conforms to bitcoins automake build system
#[derive(Debug)]
pub struct SubCommandName;

impl<'c> SubCommandName {
    pub const GENERATE: &'c str = "generate";
    pub const BUILD: &'c str = "build";
    pub const AUTH: &'c str = "auth";
    pub const FETCH: &'c str = "fetch";
}

/// Each subcommand will have associated arguments that go with it
pub struct ArgName;

impl<'c> ArgName {
    // Args for SubCommandName::GENERATE
    pub const BITCOIN: &'c str = "bitcoin";
    pub const LITECOIN: &'c str = "litecoin";
    // Args for SubCommandName::BUILD
    pub const STRATEGY: &'c str = "strategy";
    // Args for SubCommandName::AUTH
    pub const TOKEN: &'c str = "token";
    // Args for SubCommandName::FETCH
    pub const LIST_REMOTE: &'c str = "list_remote";
    pub const LIST_LOCAL: &'c str = "list_local";
    pub const LATEST: &'c str = "latest";
    pub const TAG: &'c str = "tag";
}

/// Helps distinguish betweem arguments that have values,
/// and arguments that don't.
#[derive(Debug, Clone)]
pub struct Argument {
    pub value: Option<String>,
    pub name: String,
}

impl Default for Argument {
    fn default() -> Self {
        Self {
            value: None,
            name: String::from(""),
        }
    }
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        if let Some(val) = self.value.clone() {
            return write!(f, "Argument Name: {}\nValue: {}", self.name, val,);
        }
        write!(f, "Argument Name: {}\nValue: None", self.name,)
    }
}

/// Returns the user specified command and the argument
/// structure that goes with it.
#[derive(Debug, Clone)]
pub struct ActiveCommand {
    sub_command: String,
    arg: Argument,
}

impl fmt::Display for ActiveCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Subcommand Name: {}\n{}", self.sub_command, self.arg)
    }
}

impl ActiveCommand {
    pub fn new(sub_command: &str, arg: Argument) -> Self {
        Self {
            sub_command: String::from(sub_command),
            arg,
        }
    }

    pub fn sub_command(&self) -> &String {
        &self.sub_command
    }

    pub fn arg(&self) -> Argument {
        self.arg.clone()
    }
}
