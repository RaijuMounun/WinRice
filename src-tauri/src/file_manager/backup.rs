use std::path::{Path, PathBuf};
use super::FileManagerError;

pub struct BackupManager;

use tempfile::Builder;

pub fn create_snapshot(target_path: &Path) -> Result<PathBuf, FileManagerError> {
    if !target_path.exists() {
        return Err(FileManagerError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Target file does not exist",
        )));
    }
    
    let parent = target_path.parent().unwrap_or_else(|| Path::new(""));
    let file_name = target_path.file_name().unwrap_or_default().to_string_lossy();
    
    let mut temp_file = Builder::new()
        .prefix(&format!("{}_backup_", file_name))
        .tempfile_in(parent)?;
        
    let mut original = std::fs::File::open(target_path)?;
    std::io::copy(&mut original, &mut temp_file)?;
    temp_file.as_file().sync_all()?;
    
    let (_, path) = temp_file.keep().map_err(|e| e.error)?;
    
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_create_snapshot_success() {
        let dir = tempdir().unwrap();
        let target_path = dir.path().join("config.toml");
        
        // Create initial file
        fs::write(&target_path, "test configuration data").unwrap();
        
        let result = create_snapshot(&target_path);
        assert!(result.is_ok(), "create_snapshot should succeed");
        
        let backup_path = result.unwrap();
        
        assert_ne!(backup_path, target_path, "Backup path must be different from target path");
        assert!(backup_path.exists(), "Backup path should exist");
        
        let content = fs::read(&backup_path).unwrap();
        assert_eq!(content, b"test configuration data", "Backup should contain exact original content");
        
        // Modify original file and ensure backup is unaffected
        fs::write(&target_path, "modified").unwrap();
        let backup_content = fs::read(&backup_path).unwrap();
        assert_eq!(backup_content, b"test configuration data", "Backup should not be affected by changes to original file");
    }

    #[test]
    fn test_create_snapshot_non_existent_file() {
        let dir = tempdir().unwrap();
        let target_path = dir.path().join("does_not_exist.toml");
        
        let result = create_snapshot(&target_path);
        
        // This should fail, since the file doesn't exist
        assert!(result.is_err(), "create_snapshot should fail if the target file does not exist");
    }
}
