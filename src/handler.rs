use uuid::Uuid;

use crate::error::{ServiceError, ServiceResult};
use crate::model::{CreateItemRequest, Item, UpdateItemRequest};
use crate::store::ItemStore;

/// Service layer that sits between the Lambda entrypoint and the
/// storage backend.
pub struct ItemHandler<S: ItemStore> {
    store: S,
}

impl<S: ItemStore> ItemHandler<S> {
    pub fn new(store: S) -> Self {
        Self { store }
    }

    /// Create a new item from a request payload.
    ///
    /// # Errors
    ///
    /// Returns `ServiceError::Validation` if the request fields
    /// are invalid.
    pub fn create(&self, request: CreateItemRequest) -> ServiceResult<Item> {
        let mut item = Item::new(request.name, request.owner_id)?;

        if let Some(desc) = request.description {
            item.set_description(desc);
        }

        self.store.put(&item)?;

        tracing::info!(
            id = %item.id(),
            name = %item.name(),
            "item created"
        );

        Ok(item)
    }

    /// Retrieve an item by ID.
    ///
    /// # Errors
    ///
    /// Returns `ServiceError::NotFound` if the item does not
    /// exist.
    pub fn get(&self, id: Uuid) -> ServiceResult<Item> {
        self.store.get(id)
    }

    /// List all items.
    ///
    /// # Errors
    ///
    /// Returns `ServiceError::Storage` on backend failure.
    pub fn list(&self) -> ServiceResult<Vec<Item>> {
        self.store.list()
    }

    /// Update an existing item's mutable fields.
    ///
    /// # Errors
    ///
    /// Returns `ServiceError::NotFound` if the item does not
    /// exist, or `ServiceError::Validation` if the new name is
    /// empty.
    pub fn update(&self, id: Uuid, request: UpdateItemRequest) -> ServiceResult<Item> {
        let mut item = self.store.get(id)?;

        if let Some(name) = request.name
            && name.trim().is_empty()
        {
            return Err(ServiceError::Validation(
                "name must not be empty".to_owned(),
            ));
        }

        if let Some(desc) = request.description {
            item.set_description(desc);
        }

        self.store.put(&item)?;

        tracing::info!(id = %id, "item updated");

        Ok(item)
    }

    /// Archive an item by ID.
    ///
    /// # Errors
    ///
    /// Returns `ServiceError::NotFound` if the item does not
    /// exist, or `ServiceError::Validation` if the item is
    /// already deleted.
    pub fn archive(&self, id: Uuid) -> ServiceResult<Item> {
        let mut item = self.store.get(id)?;
        item.archive()?;
        self.store.put(&item)?;

        tracing::info!(id = %id, "item archived");

        Ok(item)
    }

    /// Soft-delete an item by ID.
    ///
    /// # Errors
    ///
    /// Returns `ServiceError::NotFound` if the item does not
    /// exist.
    pub fn delete(&self, id: Uuid) -> ServiceResult<()> {
        let mut item = self.store.get(id)?;
        item.delete();
        self.store.put(&item)?;

        tracing::info!(id = %id, "item deleted");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ItemStatus;
    use crate::store::InMemoryStore;

    fn test_handler() -> ItemHandler<InMemoryStore> {
        ItemHandler::new(InMemoryStore::new())
    }

    fn create_request(name: &str, owner: &str) -> CreateItemRequest {
        CreateItemRequest {
            name: name.to_owned(),
            description: None,
            owner_id: owner.to_owned(),
        }
    }

    #[test]
    fn create_and_get() {
        let handler = test_handler();
        let item = handler.create(create_request("Test", "user-1")).unwrap();
        let retrieved = handler.get(item.id()).unwrap();
        assert_eq!(retrieved.name(), "Test");
    }

    #[test]
    fn create_with_description() {
        let handler = test_handler();
        let request = CreateItemRequest {
            name: "Test".to_owned(),
            description: Some("A description".to_owned()),
            owner_id: "user-1".to_owned(),
        };
        let item = handler.create(request).unwrap();
        assert_eq!(item.description(), Some("A description"));
    }

    #[test]
    fn create_rejects_empty_name() {
        let handler = test_handler();
        let result = handler.create(create_request("", "user-1"));
        assert!(result.is_err());
    }

    #[test]
    fn list_items() {
        let handler = test_handler();
        handler.create(create_request("One", "user-1")).unwrap();
        handler.create(create_request("Two", "user-1")).unwrap();
        let items = handler.list().unwrap();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn update_description() {
        let handler = test_handler();
        let item = handler.create(create_request("Test", "user-1")).unwrap();

        let updated = handler
            .update(
                item.id(),
                UpdateItemRequest {
                    name: None,
                    description: Some("Updated".to_owned()),
                },
            )
            .unwrap();

        assert_eq!(updated.description(), Some("Updated"));
    }

    #[test]
    fn update_nonexistent_fails() {
        let handler = test_handler();
        let result = handler.update(
            Uuid::new_v4(),
            UpdateItemRequest {
                name: None,
                description: None,
            },
        );
        assert!(matches!(result, Err(ServiceError::NotFound(_))));
    }

    #[test]
    fn archive_item() {
        let handler = test_handler();
        let item = handler.create(create_request("Test", "user-1")).unwrap();
        let archived = handler.archive(item.id()).unwrap();
        assert_eq!(archived.status(), ItemStatus::Archived);
    }

    #[test]
    fn archive_deleted_item_fails() {
        let handler = test_handler();
        let item = handler.create(create_request("Test", "user-1")).unwrap();
        handler.delete(item.id()).unwrap();
        let result = handler.archive(item.id());
        assert!(result.is_err());
    }

    #[test]
    fn delete_item() {
        let handler = test_handler();
        let item = handler.create(create_request("Test", "user-1")).unwrap();
        handler.delete(item.id()).unwrap();
        let retrieved = handler.get(item.id()).unwrap();
        assert_eq!(retrieved.status(), ItemStatus::Deleted);
    }

    #[test]
    fn get_nonexistent_fails() {
        let handler = test_handler();
        let result = handler.get(Uuid::new_v4());
        assert!(matches!(result, Err(ServiceError::NotFound(_))));
    }
}
