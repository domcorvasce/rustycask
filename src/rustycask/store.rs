use std::error;
use std::fs;
use std::path::PathBuf;

pub struct Cask {
    dir_path: PathBuf,
}

impl Cask {
    pub fn new(dir_path: &str) -> Self {
        Cask {
            dir_path: dir_path.into(),
        }
    }

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

    pub fn get(&self, _key: &str) -> Result<&str, &str> {
        Ok("")
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
}
