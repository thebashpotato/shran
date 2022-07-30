use std::env;

pub enum ShranFile {
    GhToken,
    BitcoinBuildLog,
    BitcoinBuildConfig,
    DownloadManifest,
}

pub struct ShranDefault;

impl<'a> ShranDefault {
    pub const PROGNAME: &'a str = "shran";
    pub const GH_TOKEN_FILENAME: &'a str = "gh.yaml";
    pub const BUILD_CONFIG_FILENAME: &'a str = "build.yaml";
    pub const BUILD_LOG_FILENAME: &'a str = "build.log";
    pub const BITCOIN_BASE_URL: &'a str = "https://github.com/bitcoin/bitcoin/archive/refs/tags";
    pub const FILE_EXTENSION: &'a str = ".tar.gz";
    pub const DOWNLOAD_MANIFEST_FILENAME: &'a str = "manifest.yaml";
    pub const SUPPORTED_BLOCKCHAINS: &'a [&'a str] = &["bitcoin"];

    #[inline(always)]
    pub fn config_dir() -> String {
        if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
            return format!("{}/{}", xdg, Self::PROGNAME);
        }
        format!("{}/.config/{}", env::var("HOME").unwrap(), Self::PROGNAME)
    }

    #[inline(always)]
    pub fn cache_dir() -> String {
        format!("{}/.cache/{}", env::var("HOME").unwrap(), Self::PROGNAME)
    }

    #[inline(always)]
    pub fn build_dir() -> String {
        if let Ok(here) = env::current_dir() {
            return String::from(here.to_str().unwrap());
        }
        String::from(env::current_dir().unwrap().to_str().unwrap())
    }

    #[inline(never)]
    pub fn forfile(file: ShranFile) -> String {
        match file {
            ShranFile::GhToken => {
                format!("{}/{}", Self::config_dir(), Self::GH_TOKEN_FILENAME)
            }
            ShranFile::BitcoinBuildLog => {
                format!("{}/{}", Self::build_dir(), Self::BUILD_LOG_FILENAME)
            }
            ShranFile::BitcoinBuildConfig => {
                format!("{}/{}", Self::build_dir(), Self::BUILD_CONFIG_FILENAME)
            }
            ShranFile::DownloadManifest => {
                format!(
                    "{}/{}",
                    Self::config_dir(),
                    Self::DOWNLOAD_MANIFEST_FILENAME
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ShranDefault, ShranFile};

    #[test]
    fn test_build_dir() {
        let expected = String::from(env!("CARGO_MANIFEST_DIR"));
        assert_eq!(expected, ShranDefault::build_dir());
    }

    #[test]
    fn test_build_dir_buildlog() {
        let expected: String = format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            ShranDefault::BUILD_LOG_FILENAME
        );
        assert_eq!(expected, ShranDefault::forfile(ShranFile::BitcoinBuildLog))
    }

    #[test]
    fn test_build_dir_buildconfig() {
        let expected: String = format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            ShranDefault::BUILD_CONFIG_FILENAME
        );
        assert_eq!(
            expected,
            ShranDefault::forfile(ShranFile::BitcoinBuildConfig)
        )
    }

    #[test]
    fn test_shran_config_dir() {
        let expected: String = format!("{}/.config/{}", env!("HOME"), ShranDefault::PROGNAME);
        assert_eq!(expected, ShranDefault::config_dir());
    }

    #[test]
    fn test_shran_cache_dir() {
        let expected: String = format!("{}/.cache/{}", env!("HOME"), ShranDefault::PROGNAME);
        assert_eq!(expected, ShranDefault::cache_dir());
    }
}
