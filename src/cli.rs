use clap::{crate_authors, crate_name, crate_version, App, Arg};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// Wrapper around the clap command line interface library.
///
/// # Example
///
/// ```no_run
///
/// ```
#[derive(Debug)]
pub struct Cli {
    build_file: Option<String>,
}

impl Cli {
    pub fn new() -> () {
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
    }

    pub fn build_file(self) -> Option<String> {
        self.build_file
    }
}


impl Display for Cli {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if let Some(file) = &self.build_file {
            return write!(f, "Build file: {}", file);
        }
        write!(f, "Build file is empty")
    }
}
