use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

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
        path.exists()
    }

    fn write_file(&self, path: &Path, contents: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        write!(file, "{}", contents)
    }
}

pub struct DryRunFileSystem;

impl FileSystem for DryRunFileSystem {
    fn create_dir_all(&self, path: &Path) -> std::io::Result<()> {
        println!("Would create directory: {}", path.display());
        Ok(())
    }

    fn exists(&self, path: &Path) -> bool {
        if path.exists() {
            println!("File exists: {}", path.display());
            true
        } else {
            false
        }
    }

    fn write_file(&self, path: &Path, _contents: &str) -> std::io::Result<()> {
        println!("Would create file: {}", path.display());
        Ok(())
    }
} 