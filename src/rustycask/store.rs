use std::collections::HashMap;
use std::fs::File;
use std::io::{Seek, Write};
use std::path::PathBuf;
use std::{error, fs};

const LOG_EXTENSION: &str = "log";

#[derive(Debug, PartialEq)]
pub(crate) struct KeydirEntry {
    file_id: u64,
    value_size: u64,
    value_pos: u64,
}

pub struct Cask {
    dir_path: PathBuf,
    active_file_id: u64,
    active_file: File,
    keydir: HashMap<String, KeydirEntry>,
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
            keydir: HashMap::new(),
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

    pub fn put(&mut self, key: &str, value: &str) -> Result<(), Box<dyn error::Error>> {
        let value_pos = self.active_file.stream_position().unwrap();

        self.active_file.write_all(&key.len().to_be_bytes())?;
        self.active_file.write_all(&value.len().to_be_bytes())?;
        self.active_file.write_all(key.as_bytes())?;
        self.active_file.write_all(value.as_bytes())?;
        self.active_file.flush()?;

        self.keydir.insert(
            String::from(key),
            KeydirEntry {
                file_id: self.active_file_id,
                value_pos,
                value_size: value.len() as u64,
            },
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::rustycask::store::{Cask, KeydirEntry};
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_open_creates_dir_when_missing() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().to_str().unwrap();
        let db = Cask::open(dir_path).unwrap();

        assert_eq!(db.dir_path.as_os_str(), dir_path);
        assert!(fs::exists(dir_path).unwrap_or(false));
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
        let _ = db.put("key", "value");

        let log_file_metadata = fs::metadata(db.dir_path.join("0.log")).unwrap();
        let log_file_size = log_file_metadata.len();
        assert!(log_file_size > 0, "expected > 0, got 0");
    }

    #[test]
    fn test_put_appends_data_to_log_file() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().to_str().unwrap();

        let mut db = Cask::open(dir_path).unwrap();

        assert!(db.put("key", "value").is_ok());
        assert!(db.put("key", "value2").is_ok());
    }

    #[test]
    fn test_put_updates_keydir() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().to_str().unwrap();

        let mut db = Cask::open(dir_path).unwrap();
        let _ = db.put("key", "value");
        let _ = db.put("key", "value2");

        let keydir_entry = db.keydir.get("key");
        assert_eq!(keydir_entry.is_some(), true);

        let value_pointer = keydir_entry.unwrap();
        assert_eq!(
            value_pointer,
            &KeydirEntry {
                file_id: 0,
                value_size: 6,
                value_pos: 24
            }
        );
    }
}
