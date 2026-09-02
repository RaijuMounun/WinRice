use std::path::Path;
use super::FileManagerError;
use tempfile::Builder;

pub fn rollback(target_path: &Path, backup_path: &Path) -> Result<(), FileManagerError> {
    if !backup_path.exists() {
        return Err(FileManagerError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Backup file does not exist",
        )));
    }
    
    let parent = target_path.parent().unwrap_or_else(|| Path::new(""));
    let mut temp_file = Builder::new().tempfile_in(parent)?;
    
    let mut backup = std::fs::File::open(backup_path)?;
    std::io::copy(&mut backup, &mut temp_file)?;
    temp_file.as_file().sync_all()?;
    
    temp_file.persist(target_path).map_err(|e| e.error)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_rollback_success() {
        let dir = tempdir().unwrap();
        let target_path = dir.path().join("config.toml");
        let backup_path = dir.path().join("config_backup.toml");
        
        // Setup state
        fs::write(&target_path, "corrupted data").unwrap();
        fs::write(&backup_path, "good data").unwrap();
        
        // Execute rollback
        let result = rollback(&target_path, &backup_path);
        assert!(result.is_ok(), "rollback should succeed");
        
        // Verify target has been restored
        let content = fs::read(&target_path).unwrap();
        assert_eq!(content, b"good data", "The target file should contain the backup data");
    }

    #[test]
    fn test_rollback_missing_backup() {
        let dir = tempdir().unwrap();
        let target_path = dir.path().join("config.toml");
        let backup_path = dir.path().join("missing_backup.toml");
        
        fs::write(&target_path, "some data").unwrap();
        
        let result = rollback(&target_path, &backup_path);
        assert!(result.is_err(), "rollback should fail if the backup file does not exist");
    }
}
