use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use uuid::Uuid;

use crate::error::{ServiceError, ServiceResult};
use crate::model::Item;

/// Trait for item persistence.
///
/// Implementations can back onto DynamoDB, an in-memory map, or
/// any other storage backend.
pub trait ItemStore: Send + Sync {
    fn get(&self, id: Uuid) -> ServiceResult<Item>;
    fn list(&self) -> ServiceResult<Vec<Item>>;
    fn put(&self, item: &Item) -> ServiceResult<()>;
    fn delete(&self, id: Uuid) -> ServiceResult<()>;
}

/// In-memory store for local development and testing.
#[derive(Debug, Clone, Default)]
pub struct InMemoryStore {
    items: Arc<Mutex<HashMap<Uuid, Item>>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ItemStore for InMemoryStore {
    fn get(&self, id: Uuid) -> ServiceResult<Item> {
        let items = self
            .items
            .lock()
            .map_err(|e| anyhow::anyhow!("lock poisoned: {e}"))?;
        items.get(&id).cloned().ok_or(ServiceError::NotFound(id))
    }

    fn list(&self) -> ServiceResult<Vec<Item>> {
        let items = self
            .items
            .lock()
            .map_err(|e| anyhow::anyhow!("lock poisoned: {e}"))?;
        Ok(items.values().cloned().collect())
    }

    fn put(&self, item: &Item) -> ServiceResult<()> {
        let mut items = self
            .items
            .lock()
            .map_err(|e| anyhow::anyhow!("lock poisoned: {e}"))?;
        items.insert(item.id(), item.clone());
        Ok(())
    }

    fn delete(&self, id: Uuid) -> ServiceResult<()> {
        let mut items = self
            .items
            .lock()
            .map_err(|e| anyhow::anyhow!("lock poisoned: {e}"))?;
        items
            .remove(&id)
            .map(|_| ())
            .ok_or(ServiceError::NotFound(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Item;

    fn test_store() -> InMemoryStore {
        InMemoryStore::new()
    }

    #[test]
    fn put_and_get() {
        let store = test_store();
        let item = Item::new("Test", "user-1").unwrap();
        let id = item.id();

        store.put(&item).unwrap();
        let retrieved = store.get(id).unwrap();
        assert_eq!(retrieved.name(), "Test");
    }

    #[test]
    fn get_nonexistent_returns_not_found() {
        let store = test_store();
        let result = store.get(Uuid::new_v4());
        assert!(matches!(result, Err(ServiceError::NotFound(_))));
    }

    #[test]
    fn list_returns_all_items() {
        let store = test_store();
        let item1 = Item::new("One", "user-1").unwrap();
        let item2 = Item::new("Two", "user-1").unwrap();

        store.put(&item1).unwrap();
        store.put(&item2).unwrap();

        let items = store.list().unwrap();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn list_empty_store() {
        let store = test_store();
        let items = store.list().unwrap();
        assert!(items.is_empty());
    }

    #[test]
    fn delete_existing_item() {
        let store = test_store();
        let item = Item::new("Test", "user-1").unwrap();
        let id = item.id();

        store.put(&item).unwrap();
        store.delete(id).unwrap();

        let result = store.get(id);
        assert!(matches!(result, Err(ServiceError::NotFound(_))));
    }

    #[test]
    fn delete_nonexistent_returns_not_found() {
        let store = test_store();
        let result = store.delete(Uuid::new_v4());
        assert!(matches!(result, Err(ServiceError::NotFound(_))));
    }

    #[test]
    fn put_overwrites_existing() {
        let store = test_store();
        let mut item = Item::new("Original", "user-1").unwrap();
        let id = item.id();
        store.put(&item).unwrap();

        item.set_description("Updated");
        store.put(&item).unwrap();

        let retrieved = store.get(id).unwrap();
        assert_eq!(retrieved.description(), Some("Updated"));
    }
}
