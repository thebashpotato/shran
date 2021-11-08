use clap::{crate_authors, crate_name, crate_version, App, Arg, ArgMatches};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
//use std::path::Path;

/// Wrapper around the clap command line interface library.
///
/// # Example
///
/// ```no_run
///
/// ```
pub struct Cli {
    build_file: Option<String>,
}

impl Cli {
    pub fn new() -> Self {
        let m = App::new(crate_name!())
            .author(crate_authors!())
            .version(crate_version!())
            .about("A command line tool for building a customized or vanilla version of Bitcoin")
            .arg(
                Arg::with_name("build-file")
                    .short("b")
                    .long("build-file")
                    .takes_value(true)
                    .help("Path to a `shran-build.yaml` file"),
            )
            .get_matches();

        let build_file: Option<String> = Self::check_build_file(&m);

        Self { build_file }
    }

    pub fn build_file(self) -> Option<String> {
        self.build_file
    }

    fn check_build_file(arg_matches: &ArgMatches) -> Option<String> {
        let mut file = None;
        if arg_matches.is_present("build-file") {
            file = Some(arg_matches.value_of("build-file").unwrap().to_owned());
        }
        file
    }
}

impl Display for Cli {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match &self.build_file {
            Some(file) => write!(f, "Build file: {}", file),
            None => write!(f, "Build file is empty"),
        }
    }
}

impl Debug for Cli {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match &self.build_file {
            Some(file) => write!(f, "Build file: {}", file),
            None => write!(f, "Build file is empty"),
        }
    }
}
