use serde::{Deserialize, Serialize};

#[cfg(feature = "polodb")]
use polodb_core::bson::oid::ObjectId;

/// File metadata stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    #[cfg(feature = "polodb")]
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[cfg(not(feature = "polodb"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    pub name: String,
    pub path: String,
    pub size: u64,
    pub mime_type: Option<String>,
    pub created_at: String,
    pub modified_at: String,
    pub cloud_id: Option<String>,
}

impl FileMetadata {
    pub fn new(name: String, path: String, size: u64) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: None,
            name,
            path,
            size,
            mime_type: None,
            created_at: now.clone(),
            modified_at: now,
            cloud_id: None,
        }
    }
}