///! This module defines all command helpers

/// All accepted subcommands that the shran cli accepts are
#[derive(Debug)]
pub struct SubCommandName;

impl<'c> SubCommandName {
    pub const GENERATE: &'c str = "generate";
    pub const BUILD: &'c str = "build";
    pub const AUTH: &'c str = "auth";
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
}

/// Returns the user specified command and the argument that goes with it
#[derive(Debug)]
pub struct ActiveCommand {
    sub_command: String,
    arg: String,
}

impl ActiveCommand {
    pub fn new(sub_command: &str, arg: &str) -> Self {
        Self {
            sub_command: String::from(sub_command),
            arg: String::from(arg),
        }
    }

    pub fn sub_command(&self) -> &String {
        &self.sub_command
    }

    pub fn arg(&self) -> &String {
        &self.arg
    }
}
