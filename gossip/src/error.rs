// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GossipError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] bincode::Error),
    
    #[error("Failed to verify signature")]
    InvalidSignature,

    #[error("Failed to insert value")]
    InsertFailed,
}