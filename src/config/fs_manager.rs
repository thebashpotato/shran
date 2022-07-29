use super::{ShranDefault, ShranFile};
use crate::error::ShranError;
use serde_yaml;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;

/// A wrapper around the built in filesystem utilites.
/// Manages writing, reading, and updating files and directories
/// which shran relies on.
///
/// # Example
///
pub struct FileSystemManager {
    gh_token_file: String,
}

impl FileSystemManager {
    /// Upon creating the FileSystemManager object, all shran directories
    /// will be checked for existance (config, cache, build), if they do not exist,
    /// they will be created. Note that only the directories will be created, not
    /// the files that live inside them.
    pub fn new() -> std::io::Result<Self> {
        if !Path::new(ShranDefault::config_dir().as_str()).exists() {
            fs::create_dir(ShranDefault::config_dir())?;
        }
        if !Path::new(ShranDefault::cache_dir().as_str()).exists() {
            fs::create_dir(ShranDefault::cache_dir())?;
        }
        Ok(Self {
            gh_token_file: ShranDefault::forfile(ShranFile::GhToken),
        })
    }

    /// Writes the users github token to a yaml file.
    /// Will trample the previous contents
    pub fn write_token(&self, token: String) -> Result<(), Box<dyn Error>> {
        if !Path::new(self.gh_token_file.as_str()).exists() {
            File::create(self.gh_token_file.as_str())?;
        }
        let mut map: BTreeMap<&str, &str> = BTreeMap::new();
        map.insert("token", token.as_str());
        let yaml_string = serde_yaml::to_string(&map)?;
        fs::write(self.gh_token_file.as_str(), yaml_string)?;

        Ok(())
    }

    /// Read the token from disk
    pub fn read_token(&self) -> Result<String, Box<dyn Error>> {
        if !Path::new(&self.gh_token_file).exists() {
            return Err(Box::new(ShranError::GithubTokenNotFoundError {
                msg: format!("{} not found", &self.gh_token_file),
                file: file!(),
                line: line!(),
                column: column!(),
            }));
        }
        let yaml = fs::read_to_string(&self.gh_token_file)?;
        let deserialized: BTreeMap<String, String> = serde_yaml::from_str(&yaml)?;
        match deserialized.get("token") {
            Some(t) => {
                return Ok(t.to_owned());
            }
            None => {
                return Err(Box::new(ShranError::GithubTokenReadError {
                    msg: format!("Failed deserializing {}", &self.gh_token_file),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                }));
            }
        }
    }
}
