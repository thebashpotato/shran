use super::archive::{Archiver, TapeArchive};
use super::GithubAuth;
use crate::error::ShranError;
use crate::{ShranDefault, ShranFile};
use serde_yaml;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::Write;
use std::path::Path;

/// Enumeration which will tell reading/writing functions
/// where to save uncompressed source trees.
pub enum BlockchainKind {
    Bitcoin,
}

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

        // create download cache directories for all supported blockchians
        for blockchain in ShranDefault::SUPPORTED_BLOCKCHAINS {
            let path = format!("{}/{}", ShranDefault::cache_dir(), *blockchain);
            if !Path::new(path.as_str()).exists() {
                fs::create_dir(path)?;
            }
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
    /// Retuns a yaml serialization error if Token cannot be serialized
    pub fn write_token(&self, token: String) -> Result<(), Box<dyn Error>> {
        if !Path::new(self.gh_token_file.as_str()).exists() {
            File::create(self.gh_token_file.as_str())?;
        }
        let yaml_string = serde_yaml::to_string(&GithubAuth::new(&token))?;
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
    /// Retuns a yaml deserialization error if Token cannot be deserialized,
    /// if this happens, it likely means the user has tampered with, or intentionally
    /// messed up the file structre.
    ///
    /// TODO: Write tests to mimic file tampering
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
        let deserialized: GithubAuth = serde_yaml::from_str(&yaml)?;
        Ok(deserialized.extract_token())
    }

    /// This function writes an archive file to disk for a specified blockchain
    /// to the `~/.cache/shran/<BlockchainKind>` directory, then extracts the contents,
    /// and removes the archive file when it is done.
    ///
    /// # Parms
    ///
    /// 1. filename: name of the file with no path attached
    /// 2. file_bytes: The actual contents of the file as bytes
    /// 3. BlockchainKind: Enum representing the blockchain type (bitcoin, litecoin etc..)
    ///
    /// # Errors
    ///
    /// Returns ShranError::BlockchainVersionAlreadyExistsError if the archive has
    /// already been downloaded
    ///
    /// Returns a variety of fs module errors if file creation fails, or if removing
    /// the archive file afterwards fails
    pub fn write_and_extract_blockchain_archive(
        &self,
        filename: &str,
        file_bytes: Vec<u8>,
        blockchain_kind: BlockchainKind,
    ) -> Result<(), Box<dyn Error>> {
        match blockchain_kind {
            BlockchainKind::Bitcoin => {
                let abs_dir = format!("{}/bitcoin", ShranDefault::cache_dir());
                let archive_file_path = format!("{}/{}", abs_dir, filename);
                if Path::new(archive_file_path.as_str()).exists() {
                    return Err(Box::new(ShranError::BlockchainVersionAlreadyExistsError {
                        msg: format!("{} already exists", archive_file_path),
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    }));
                }
                // write the archive file to disk
                let mut file = File::create(&archive_file_path)?;
                file.write_all(file_bytes.as_slice())?;
                // deflate and extract the archive
                TapeArchive::new(archive_file_path.as_str(), abs_dir.as_str()).unpack()?;
                // remove the archive file as we no longer require it
                fs::remove_file(archive_file_path)?;
            }
        }
        Ok(())
    }
}
