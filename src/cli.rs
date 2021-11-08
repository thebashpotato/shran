use crate::error::{ShranDynamicError, ShranError};
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches};
use std::path::Path;

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
    build_file: Option<String>,
    gen_config: bool,
    with_token: Option<String>,
}

impl<'ebuf> Cli {
    pub fn new() -> ShranDynamicError<'ebuf, Self> {
        let m = App::new(crate_name!())
            .author(crate_authors!())
            .version(crate_version!())
            .about(crate_description!())
            .arg(
                Arg::with_name("build-file")
                    .short("b")
                    .long("build-file")
                    .takes_value(true)
                    .help("Path to a `shran-build.yaml` file"),
            )
            .get_matches();

        let build_file = Self::validate_build_file(&m)?;

        Ok(Self {
            build_file,
            gen_config: false,
            with_token: None,
        })
    }

    pub fn build_file(self) -> Option<String> {
        self.build_file
    }

    pub fn gen_config(self) -> bool {
        self.gen_config
    }

    pub fn with_token(self) -> Option<String> {
        self.with_token
    }

    fn validate_build_file(arg_matches: &ArgMatches) -> Result<Option<String>, ShranError<'ebuf>> {
        let mut build_file: Option<String> = None;
        if arg_matches.is_present("build-file") {
            let bfile = arg_matches.value_of("build-file").unwrap().to_owned();
            if !Path::new(&bfile).exists() {
                return Err(ShranError::BuildFileError {
                    found: bfile.to_string(),
                    file: file!(),
                    line: line!(),
                });
            }
            build_file = Some(bfile);
        }
        Ok(build_file)
    }
}
