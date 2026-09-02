use std::path::Path;
use super::FileManagerError;

use std::io::Write;
use tempfile::Builder;

pub fn write_atomically(target_path: &Path, content: &[u8]) -> Result<(), FileManagerError> {
    let parent = target_path.parent().unwrap_or_else(|| Path::new(""));
    let mut temp_file = Builder::new().tempfile_in(parent)?;
    temp_file.write_all(content)?;
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
    fn test_write_atomically_success() {
        let dir = tempdir().unwrap();
        let target_path = dir.path().join("target.txt");
        let link_path = dir.path().join("link.txt");
        
        // Write some initial content
        fs::write(&target_path, b"old content").unwrap();
        // Create a hard link to verify atomicity
        fs::hard_link(&target_path, &link_path).unwrap();
        
        // Try atomic write
        let result = write_atomically(&target_path, b"new content");
        assert!(result.is_ok(), "write_atomically should succeed");
        
        // Verify the file contains the new content
        let content = fs::read(&target_path).unwrap();
        assert_eq!(content, b"new content", "The file content should be overwritten atomically");
        
        // Verify the hard link still has old content (proves the file was replaced, not overwritten in place)
        let link_content = fs::read(&link_path).unwrap();
        assert_eq!(link_content, b"old content", "Atomic write should break hard links by replacing the file, not overwriting in place");
        
        // Check for leftover temp files in the directory
        let entries = fs::read_dir(dir.path()).unwrap();
        let file_count = entries.count();
        // 2 files expected: target.txt and link.txt
        assert_eq!(file_count, 2, "There should be no leftover temporary files in the directory");
    }

    #[test]
    fn test_write_atomically_parent_dir_missing() {
        let dir = tempdir().unwrap();
        let target_path = dir.path().join("missing_dir").join("target.txt");
        
        let result = write_atomically(&target_path, b"new content");
        assert!(result.is_err(), "write_atomically should fail if the parent directory doesn't exist");
    }
}
