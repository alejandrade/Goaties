use serde::{Deserialize, Serialize};

/// Request to create a new file metadata entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileRequest {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub mime_type: Option<String>,
}

/// Request to update file metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFileRequest {
    pub name: Option<String>,
    pub path: Option<String>,
    pub size: Option<u64>,
    pub mime_type: Option<String>,
}

/// Request to search files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilesRequest {
    pub query: String,
}