use thiserror::Error;

pub mod atomic;
pub mod backup;
pub mod rollback;

#[derive(Error, Debug)]
pub enum FileManagerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Backup error: {0}")]
    BackupError(String),
    #[error("Rollback error: {0}")]
    RollbackError(String),
}

pub use atomic::write_atomically;
pub use backup::create_snapshot;
pub use rollback::rollback;
