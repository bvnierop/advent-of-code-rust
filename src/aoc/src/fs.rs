use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::collections::HashSet;
use std::path::PathBuf;
use std::cell::RefCell;

#[cfg_attr(test, mockall::automock)]
pub trait FileSystem {
    fn create_dir_all(&self, path: &Path) -> std::io::Result<()>;
    fn exists(&self, path: &Path) -> bool;
    fn write_file(&self, path: &Path, contents: &str) -> std::io::Result<()>;
}

pub struct RealFileSystem;

impl FileSystem for RealFileSystem {
    fn create_dir_all(&self, path: &Path) -> std::io::Result<()> {
        fs::create_dir_all(path)
    }

    fn exists(&self, path: &Path) -> bool {
        if path.exists() {
            println!("File exists: {}", path.display());
            true
        } else {
            false
        }
    }

    fn write_file(&self, path: &Path, contents: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        write!(file, "{}", contents)?;
        println!("Created file: {}", path.display());
        Ok(())
    }
}

pub struct DryRunFileSystem {
    created_paths: RefCell<HashSet<PathBuf>>,
}

impl Default for DryRunFileSystem {
    fn default() -> Self {
        Self {
            created_paths: RefCell::new(HashSet::new()),
        }
    }
}

impl DryRunFileSystem {
    pub fn new() -> Self {
        Self::default()
    }
}

impl FileSystem for DryRunFileSystem {
    fn create_dir_all(&self, path: &Path) -> std::io::Result<()> {
        println!("Would create directory: {}", path.display());
        self.created_paths.borrow_mut().insert(path.into());
        Ok(())
    }

    fn exists(&self, path: &Path) -> bool {
        if path.exists() || self.created_paths.borrow().contains(path) {
            true
        } else {
            false
        }
    }

    fn write_file(&self, path: &Path, _contents: &str) -> std::io::Result<()> {
        println!("Would create file: {}", path.display());
        self.created_paths.borrow_mut().insert(path.into());
        Ok(())
    }
}