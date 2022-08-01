use crate::ShranError;
use crate::{ShranDefault, ShranFile};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct ManifestEntry {
    pub version: String,
    pub published_date: String,
    pub installation_location: String,
}

impl ManifestEntry {
    pub fn new(version: String, published_date: String, installation_location: String) -> Self {
        Self {
            version,
            published_date,
            installation_location,
        }
    }
}

pub type BlockchainDescription = String;
pub type Manifest = HashMap<BlockchainDescription, ManifestEntry>;

pub struct ManifestManager {
    manifest_file: String,
    entries: Manifest,
}

impl ManifestManager {
    /// Creates a new ManifestManager object, the manifest file is checked for
    /// existance, and is created if it is not.
    ///
    /// # Errors
    /// yaml Errors, and std lib io Errors can be returned
    ///
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let manifest_file = ShranDefault::forfile(ShranFile::ManifestFile);
        if !Path::new(&manifest_file).exists() {
            fs::File::create(&manifest_file)?;
        }
        // load entries here
        let yaml = fs::read_to_string(&manifest_file)?;
        let mut entries: Manifest = HashMap::new();
        if !yaml.is_empty() {
            entries = serde_yaml::from_str(&yaml)?;
        }

        Ok(Self {
            manifest_file,
            entries,
        })
    }

    /// Returns the length of the internal Manifest entries hash map
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Adds an entry to the manfist.yaml file
    ///
    /// # Params
    /// 1. key the full description of the blockchain [e.g.] Bitcoin core v21.0
    /// 2. Manifest entry object
    ///
    /// # Errors
    /// Can throw std lib io Errors, or ShranError::ManifestEntry
    pub fn add_entry(
        &mut self,
        key: BlockchainDescription,
        entry: &ManifestEntry,
    ) -> Result<(), Box<dyn Error>> {
        if !self.entries.contains_key(&key) {
            self.entries.insert(key, entry.to_owned());
            // write the updated manifest to disk
            let entries_string = serde_yaml::to_string(&self.entries)?;
            fs::write(self.manifest_file.as_str(), entries_string)?;
            return Ok(());
        }
        Err(Box::new(ShranError::ManifestEntryError {
            msg: format!("{} aleady exists in manifest file", key),
            file: file!(),
            line: line!(),
            column: column!(),
        }))
    }

    /// Removes an entry to the manfist.yaml file
    ///
    /// # Params
    /// 1. key: The full description of the blockchain [e.g.] Bitcoin core v21.0
    ///
    /// # Errors
    /// Can throw std lib io Errors, or ShranError::ManifestEntry
    pub fn remove_entry(
        &mut self,
        key: BlockchainDescription,
    ) -> Result<ManifestEntry, Box<dyn Error>> {
        if self.entries.contains_key(&key) {
            if let Some(entry) = self.entries.remove(&key) {
                let entries_string = serde_yaml::to_string(&self.entries)?;
                fs::write(self.manifest_file.as_str(), entries_string)?;
                return Ok(entry);
            }
        }
        Err(Box::new(ShranError::ManifestEntryError {
            msg: format!("{} does not exist in manifest file", key),
            file: file!(),
            line: line!(),
            column: column!(),
        }))
    }

    /// Get an refrence to an entry from the manifest.yaml file,
    ///
    /// # Params
    /// 1. key: The full description of the blockchain [e.g.] Bitcoin core v21.0
    ///
    /// # Errors
    /// Can throw std lib io Errors, or ShranError::ManifestEntry
    pub fn get_entry(&self, key: BlockchainDescription) -> Result<&ManifestEntry, Box<dyn Error>> {
        if self.entries.contains_key(&key) {
            if let Some(entry) = self.entries.get(&key) {
                return Ok(entry);
            }
        }

        Err(Box::new(ShranError::ManifestEntryError {
            msg: format!("{} does not exist in manifest file", key),
            file: file!(),
            line: line!(),
            column: column!(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::{ManifestEntry, ManifestManager};
    use crate::{ShranDefault, ShranFile};
    use std::collections::HashMap;
    use std::fs;

    #[test]
    fn test_manifest_manager_1_new() {
        match ManifestManager::new() {
            Ok(_) => {
                assert!(true);
            }
            Err(e) => {
                eprintln!("{}", e);
                assert!(false)
            }
        }
        let _ = fs::remove_file(ShranDefault::forfile(ShranFile::ManifestFile));
    }

    #[test]
    fn test_manifest_manager_add_entry() {
        match ManifestManager::new() {
            Ok(mut mm) => {
                let blck_desc = String::from("Bitcoin core v23.0");
                let entry = ManifestEntry::new(
                    "v23.0".to_string(),
                    "2022-04-25 14:17:32 UTC".to_string(),
                    "/home/matt/.cache/shra/bitcoin/bitcoin-23.0".to_string(),
                );
                if let Err(e) = mm.add_entry(blck_desc, &entry) {
                    eprint!("{}", e);
                    assert!(false);
                } else {
                    assert!(true);
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                assert!(false, "ManifestManager::new() failed")
            }
        }
        let _ = fs::remove_file(ShranDefault::forfile(ShranFile::ManifestFile));
    }

    #[test]
    fn test_manifest_manager_add_multiple_entries() {
        match ManifestManager::new() {
            Ok(mut mm) => {
                let mut test_entries: HashMap<&str, ManifestEntry> = HashMap::new();
                test_entries.insert(
                    "Bitcoin core v23.0",
                    ManifestEntry::new(
                        "v23.0".to_string(),
                        "2022-04-25 14:17:32 UTC".to_string(),
                        "/home/matt/.cache/shran/bitcoin/bitcoin-23.0".to_string(),
                    ),
                );
                test_entries.insert(
                    "Bitcoin core v22.0",
                    ManifestEntry::new(
                        "v22.0".to_string(),
                        "2022-04-25 14:17:32 UTC".to_string(),
                        "/home/matt/.cache/shran/bitcoin/bitcoin-22.0".to_string(),
                    ),
                );
                test_entries.insert(
                    "Bitcoin core v21.0",
                    ManifestEntry::new(
                        "v21.0".to_string(),
                        "2022-04-25 14:17:32 UTC".to_string(),
                        "/home/matt/.cache/shran/bitcoin/bitcoin-21.0".to_string(),
                    ),
                );
                for (key, value) in &test_entries {
                    if let Ok(()) = mm.add_entry(key.to_string(), value) {
                        assert!(true);
                    } else {
                        assert!(false, "failed adding entry");
                    }
                }
                assert_eq!(test_entries.len(), mm.len());
            }
            Err(e) => {
                eprintln!("{}", e);
                assert!(false)
            }
        }
    }

    #[test]
    fn test_manifest_manager_get_entry() {
        match ManifestManager::new() {
            Ok(mm) => {
                if let Ok(entry) = mm.get_entry("Bitcoin core v21.0".to_string()) {
                    assert_eq!(entry.version, "v21.0".to_string());
                    assert_eq!(entry.published_date, "2022-04-25 14:17:32 UTC".to_string());
                    assert_eq!(
                        entry.installation_location,
                        "/home/matt/.cache/shran/bitcoin/bitcoin-21.0".to_string()
                    );
                } else {
                    assert!(false, "failed getting entry");
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                assert!(false)
            }
        }
    }

    #[test]
    fn test_manifest_manager_remove_entry() {
        match ManifestManager::new() {
            Ok(mut mm) => match mm.remove_entry("Bitcoin core v21.0".to_string()) {
                Ok(entry) => {
                    assert_eq!(entry.version, "v21.0".to_string());
                    assert_eq!(entry.published_date, "2022-04-25 14:17:32 UTC".to_string());
                    assert_eq!(
                        entry.installation_location,
                        "/home/matt/.cache/shran/bitcoin/bitcoin-21.0".to_string()
                    );
                }
                Err(e) => {
                    eprintln!("{}", e);
                    assert!(false);
                }
            },
            Err(e) => {
                eprintln!("{}", e);
                assert!(false)
            }
        }
        let _ = fs::remove_file(ShranDefault::forfile(ShranFile::ManifestFile));
    }
}
