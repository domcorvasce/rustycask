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

    pub fn open(dir_path: &str) -> Result<Cask, &str> {
        match fs::exists(dir_path) {
            Ok(true) => Ok(Self::new(dir_path)),
            Ok(false) => {
                let _ = fs::create_dir(dir_path);
                Ok(Self::new(dir_path))
            }
            Err(_) => Err("Cannot create directory"),
        }
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
}
