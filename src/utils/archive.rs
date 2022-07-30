extern crate tar;
use flate2::read::GzDecoder;
use std::error::Error;
use std::fs::File;

/// Archiver is a trait that a zip or tar struct must implement
pub trait Archiver<T> {
    /// Unpack a given archive file to a destination.
    /// The archived file and the destination location must
    /// must be given to whatever structs new function that implements
    /// this trait.
    fn unpack(&self) -> Result<T, Box<dyn Error>>;
}

/// TapeArchive uses flate2 and tar library to deal with
/// tar archives which use gnu extensions (tar.gz).
pub struct TapeArchive<'archive> {
    archive: &'archive str,
    destination_dir: &'archive str,
}

impl<'archive> TapeArchive<'archive> {
    /// Build a new TapeArchive object
    ///
    /// # Params
    ///
    /// 1. archive: The file that needs to be decompressed (file.tar.gz)
    ///
    /// 2. destination_dir: The absolute path to destination directory
    ///    where the archive should be dumped
    pub fn new(archive: &'archive str, destination_dir: &'archive str) -> Self {
        Self {
            archive,
            destination_dir,
        }
    }
}

impl<'archive> Archiver<()> for TapeArchive<'archive> {
    fn unpack(&self) -> Result<(), Box<dyn Error>> {
        let file: File = File::open(&self.archive)?;
        let tar = GzDecoder::new(file);
        tar::Archive::new(tar).unpack(&self.destination_dir)?;
        Ok(())
    }
}
