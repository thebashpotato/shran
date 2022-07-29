use super::{ShranDefault, ShranFile};
use crate::error::ShranError;
use serde_yaml;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;

/// A wrapper around the built in filesystem utilites.
/// Manages writing, reading, and updating files and directories
/// which shran relies on.
pub struct FileSystemManager {
    gh_token_file: String,
}

impl FileSystemManager {
    /// Upon creating the FileSystemManager object, all shran directories
    /// will be checked for existance (config, cache, build), if they do not exist,
    /// they will be created. Note that only the directories will be created, not
    /// the files that live inside them.
    ///
    /// # Errors
    /// Returns an io::Error if creating the directories fails
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
    /// Will trample the previous contents of the file.
    ///
    /// # Errors
    ///
    /// Returns an io::Error of file creation fails, or file writing
    /// fails
    ///
    /// Retuns a yaml serialization error if the HashMap cannot
    /// be serialized
    pub fn write_token(&self, token: String) -> Result<(), Box<dyn Error>> {
        if !Path::new(self.gh_token_file.as_str()).exists() {
            File::create(self.gh_token_file.as_str())?;
        }
        let mut map: HashMap<&str, &str> = HashMap::new();
        let _ = map.insert("token", token.as_str());
        let yaml_string = serde_yaml::to_string(&map)?;
        fs::write(self.gh_token_file.as_str(), yaml_string)?;

        Ok(())
    }

    /// Read the token from disk, returns a moved String object
    /// containing said token for github authentication purposes
    ///
    /// # Errors
    ///
    /// Returns ShranError::GithubTokenNotFoundError if gh.yaml file
    /// is not found on disk.
    ///
    /// Returns ShranError::GithubTokenReadError if deserialzing the yaml
    /// fails.
    ///
    /// There are possibillities for std lib fs errors being thrown,
    /// which is why the error handling is dispatched dynamically instead
    /// of statically.
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
        let deserialized: HashMap<String, String> = serde_yaml::from_str(&yaml)?;
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
