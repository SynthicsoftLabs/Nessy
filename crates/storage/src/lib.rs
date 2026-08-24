// Copyright 2026 Synthicsoft Labs LLC
// Licensed under the Apache License, Version 2.0.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ContentId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredObject {
    pub id: ContentId,
    pub media_type: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("invalid content identifier")]
    InvalidContentId,
    #[error("object not found")]
    NotFound,
    #[error("content integrity verification failed")]
    IntegrityFailure,
}

#[derive(Clone, Default)]
pub struct ContentStore {
    objects: Arc<RwLock<HashMap<ContentId, StoredObject>>>,
}

impl ContentStore {
    pub async fn put(&self, media_type: impl Into<String>, data: Vec<u8>) -> ContentId {
        let digest = Sha256::digest(&data);
        let id = ContentId(format!("sha256:{}", hex(&digest)));
        let object = StoredObject {
            id: id.clone(),
            media_type: media_type.into(),
            data,
        };
        self.objects.write().await.insert(id.clone(), object);
        id
    }

    pub async fn get(&self, id: &ContentId) -> Result<StoredObject, StorageError> {
        let object = self
            .objects
            .read()
            .await
            .get(id)
            .cloned()
            .ok_or(StorageError::NotFound)?;
        let digest = Sha256::digest(&object.data);
        if object.id.0 != format!("sha256:{}", hex(&digest)) {
            return Err(StorageError::IntegrityFailure);
        }
        Ok(object)
    }

    pub async fn contains(&self, id: &ContentId) -> bool {
        self.objects.read().await.contains_key(id)
    }
}

fn hex(bytes: &[u8]) -> String {
    const TABLE: &[u8; 16] = b"0123456789abcdef";
    bytes
        .iter()
        .flat_map(|b| {
            [
                TABLE[(b >> 4) as usize] as char,
                TABLE[(b & 0x0f) as usize] as char,
            ]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn content_is_deduplicated_by_hash() {
        let store = ContentStore::default();
        let a = store.put("text/plain", b"hello".to_vec()).await;
        let b = store.put("text/plain", b"hello".to_vec()).await;
        assert_eq!(a, b);
        assert_eq!(store.get(&a).await.unwrap().data, b"hello");
    }
}
