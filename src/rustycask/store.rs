use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{error, fs};

const LOG_EXTENSION: &str = "log";

pub struct Cask {
    dir_path: PathBuf,
    active_file_id: u64,
    active_file: File,
}

impl Cask {
    /// Note: The method assumes that the directory exists
    pub fn new(dir_path: &str) -> Self {
        let dir_path = PathBuf::from(dir_path);

        // Data files are named in numerical order (e.g. 0.log, 1.log, and so on)
        // We retrieve the ID (file stem) of the latest data file in order to write the next one
        let last_active_file_id = fs::read_dir(&dir_path)
            .unwrap()
            .filter_map(|entry| {
                let entry_path = entry.unwrap().path();
                match entry_path.is_file() && entry_path.extension()?.to_str()? == LOG_EXTENSION {
                    true => Some(entry_path.file_stem()?.to_str()?.parse::<u64>().ok()?),
                    _ => None,
                }
            })
            .max();

        let active_file_id = match last_active_file_id {
            Some(value) => value + 1,
            None => 0,
        };

        let active_file =
            File::create_new(dir_path.join(format!("{}.{}", active_file_id, LOG_EXTENSION)))
                .unwrap();

        Cask {
            dir_path: dir_path,
            active_file_id: active_file_id,
            active_file: active_file,
        }
    }

    /// Open a new or existing cask directory
    pub fn open(dir_path: &str) -> Result<Cask, Box<dyn error::Error>> {
        match fs::exists(dir_path) {
            Ok(true) => Ok(Self::new(dir_path)),
            Ok(false) => {
                fs::create_dir(dir_path).unwrap();
                Ok(Self::new(dir_path))
            }
            Err(error) => Err(Box::new(error)),
        }
    }

    pub fn put(&mut self, key: &[u8], value: &[u8]) -> Result<(), Box<dyn error::Error>> {
        self.active_file.write_all(&key.len().to_be_bytes())?;
        self.active_file.write_all(&value.len().to_be_bytes())?;
        self.active_file.write_all(key)?;
        self.active_file.write_all(value)?;
        self.active_file.flush()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::rustycask::store::Cask;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_open_creates_dir_when_missing() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().to_str().unwrap();
        let db = Cask::open(dir_path).unwrap();

        assert_eq!(db.dir_path.as_os_str(), dir_path);
        assert_eq!(fs::exists(dir_path).unwrap(), true);
    }

    #[test]
    fn test_open_initializes_data_file_on_empty_dir() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().to_str().unwrap();
        let db = Cask::open(dir_path).unwrap();
        assert_eq!(db.active_file_id, 0);
    }

    #[test]
    fn test_put_writes_data_to_log_file() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().to_str().unwrap();

        let mut db = Cask::open(dir_path).unwrap();
        let _ = db.put(b"key", b"value");

        let log_file_metadata = fs::metadata(db.dir_path.join("0.log")).unwrap();
        let log_file_size = log_file_metadata.len();
        assert!(log_file_size > 0, "expected > 0, got 0");
    }

    #[test]
    fn test_put_appends_data_to_log_file() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().to_str().unwrap();

        let mut db = Cask::open(dir_path).unwrap();

        assert!(db.put(b"key", b"value").unwrap() == ());
        assert!(db.put(b"key", b"value2").unwrap() == ());
    }
}
