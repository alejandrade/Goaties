use serde::{Deserialize, Serialize};
use crate::FileMetadata;

/// Health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
}

/// Response containing a single file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileResponse {
    pub file: FileMetadata,
}

/// Response containing multiple files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesResponse {
    pub files: Vec<FileMetadata>,
    pub total: usize,
}

/// Generic error response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}