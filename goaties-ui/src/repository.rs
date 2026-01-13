use anyhow::Result;
use polodb_core::{bson::{doc, oid::ObjectId}, Collection, CollectionT};

use crate::models::FileMetadata;

pub struct FileRepository {
    db: polodb_core::Database,
}

impl FileRepository {
    pub fn new(db_path: &str) -> Result<Self> {
        let db = polodb_core::Database::open_path(db_path)?;
        Ok(Self { db })
    }

    fn collection(&self) -> Collection<FileMetadata> {
        self.db.collection::<FileMetadata>("file_metadata")
    }

    pub fn create(&self, file: FileMetadata) -> Result<FileMetadata> {
        let result = self.collection().insert_one(file)?;

        // Get the inserted document back
        let id = result.inserted_id.as_object_id()
            .ok_or_else(|| anyhow::anyhow!("Failed to get inserted ID"))?;

        self.get_by_id(&id.to_hex())
            .and_then(|opt| opt.ok_or_else(|| anyhow::anyhow!("Failed to retrieve created document")))
    }

    pub fn get_by_id(&self, id: &str) -> Result<Option<FileMetadata>> {
        let oid = ObjectId::parse_str(id)?;
        let doc = self.collection().find_one(doc! { "_id": oid })?;
        Ok(doc)
    }

    pub fn get_all(&self) -> Result<Vec<FileMetadata>> {
        let cursor = self.collection().find(doc! {}).run()?;
        let files: Vec<FileMetadata> = cursor.collect::<Result<Vec<_>, _>>()?;
        Ok(files)
    }

    pub fn get_by_path(&self, path: &str) -> Result<Option<FileMetadata>> {
        let doc = self.collection().find_one(doc! { "path": path })?;
        Ok(doc)
    }

    pub fn update(&self, id: &str, file: FileMetadata) -> Result<()> {
        let oid = ObjectId::parse_str(id)?;
        let update_doc = polodb_core::bson::to_document(&file)?;

        self.collection().update_one(
            doc! { "_id": oid },
            doc! { "$set": update_doc }
        )?;

        Ok(())
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        let oid = ObjectId::parse_str(id)?;
        self.collection().delete_one(doc! { "_id": oid })?;
        Ok(())
    }

    pub fn search_by_name(&self, name_pattern: &str) -> Result<Vec<FileMetadata>> {
        // PoloDB supports regex queries - no SQL needed!
        let cursor = self.collection().find(doc! {
            "name": { "$regex": name_pattern }
        }).run()?;

        let files: Vec<FileMetadata> = cursor.collect::<Result<Vec<_>, _>>()?;
        Ok(files)
    }
}