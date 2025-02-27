use std::path::PathBuf;
use std::{error, fs};

const LOG_EXTENSION: &str = "log";

pub struct Cask {
    dir_path: PathBuf,
    active_file_id: u64,
}

impl Cask {
    /// Note: The method assumes that the directory exists
    pub fn new(dir_path: &str) -> Self {
        // Data files are named in numerical order (e.g. 0.log, 1.log, and so on)
        // We retrieve the ID (file stem) of the latest data file in order to write the next one
        let last_active_file_id = fs::read_dir(dir_path)
            .unwrap()
            .filter_map(|entry| {
                let entry_path = entry.unwrap().path();
                match entry_path.is_file() && entry_path.extension()?.to_str()? == LOG_EXTENSION {
                    true => Some(entry_path.file_stem()?.to_str()?.parse::<u64>().ok()?),
                    _ => None,
                }
            })
            .max()
            .unwrap();

        Cask {
            dir_path: dir_path.into(),
            active_file_id: last_active_file_id + 1,
        }
    }

    /// Open a new or existing cask directory
    pub fn open(dir_path: &str) -> Result<Cask, Box<dyn error::Error>> {
        match fs::exists(dir_path) {
            Ok(true) => Ok(Self::new(dir_path)),
            Ok(false) => {
                Self::create_dir(dir_path).unwrap();
                Ok(Self::new(dir_path))
            }
            Err(error) => Err(Box::new(error)),
        }
    }

    /// Initialize a new cask directory including the first data file
    fn create_dir(dir_path: &str) -> Result<(), Box<dyn error::Error>> {
        let dir_path = PathBuf::from(dir_path);
        let data_filename = dir_path.join("0.log");

        fs::create_dir(dir_path)?;
        fs::File::create(data_filename)?;

        Ok(())
    }

    pub fn merge(_dir_path: &str) -> Result<(), &str> {
        Ok(())
    }

    pub fn get(&self, _key: &str) -> Option<&str> {
        None
    }

    pub fn put(&self, _key: &str, _value: &str) -> Result<(), &str> {
        Ok(())
    }

    pub fn delete(&self, _key: &str) -> Result<(), &str> {
        Ok(())
    }

    pub fn sync(&self) -> Result<(), &str> {
        Ok(())
    }

    pub fn close(&self) -> Result<(), &str> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::rustycask::store::Cask;
    use std::fs;

    #[test]
    fn test_open_creates_dir_when_missing() {
        let dir_path = "/tmp/sample_dir";

        fs::remove_dir(dir_path).unwrap_or(());
        let db = Cask::open(dir_path).unwrap();

        assert_eq!(db.dir_path.as_os_str(), dir_path);
        assert_eq!(fs::exists(dir_path).unwrap(), true);

        fs::remove_dir(dir_path).unwrap_or(());
    }

    #[test]
    fn test_open_initializes_data_file_on_empty_dir() {
        let dir_path = "/tmp/sample_dir1";

        fs::remove_dir(dir_path).unwrap_or(());
        let db = Cask::open(dir_path).unwrap();

        assert_eq!(fs::exists(db.dir_path.join("0.log")).unwrap(), true);
        fs::remove_dir(dir_path).unwrap_or(());
    }

    #[test]
    fn test_open_on_existing_dir_does_not_write_existing_files() {
        let dir_path = format!("{}/tests/fixtures/sample_db", env!("CARGO_MANIFEST_DIR"));

        let db = Cask::open(&dir_path).unwrap();
        assert_eq!(db.active_file_id, 2);
    }
}
