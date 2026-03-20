use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{ServiceError, ServiceResult};

/// The status of an item in the system.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemStatus {
    #[default]
    Active,
    Archived,
    Deleted,
}

/// A domain item managed by the service.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    id: Uuid,
    name: String,
    description: Option<String>,
    status: ItemStatus,
    owner_id: String,
}

impl Item {
    /// Create a new item with validation.
    ///
    /// # Errors
    ///
    /// Returns `ServiceError::Validation` if name is empty or
    /// owner_id is empty.
    pub fn new(name: impl Into<String>, owner_id: impl Into<String>) -> ServiceResult<Self> {
        let name = name.into();
        let owner_id = owner_id.into();

        if name.trim().is_empty() {
            return Err(ServiceError::Validation(
                "name must not be empty".to_owned(),
            ));
        }
        if owner_id.trim().is_empty() {
            return Err(ServiceError::Validation(
                "owner_id must not be empty".to_owned(),
            ));
        }

        Ok(Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            status: ItemStatus::default(),
            owner_id,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn status(&self) -> ItemStatus {
        self.status
    }

    pub fn owner_id(&self) -> &str {
        &self.owner_id
    }

    pub fn set_description(&mut self, description: impl Into<String>) {
        self.description = Some(description.into());
    }

    /// Archive the item.
    ///
    /// # Errors
    ///
    /// Returns `ServiceError::Validation` if the item is already
    /// deleted.
    pub fn archive(&mut self) -> ServiceResult<()> {
        if self.status == ItemStatus::Deleted {
            return Err(ServiceError::Validation(
                "cannot archive a deleted item".to_owned(),
            ));
        }
        self.status = ItemStatus::Archived;
        Ok(())
    }

    /// Soft-delete the item.
    pub fn delete(&mut self) {
        self.status = ItemStatus::Deleted;
    }
}

/// Request payload for creating a new item.
#[derive(Debug, Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
    pub description: Option<String>,
    pub owner_id: String,
}

/// Request payload for updating an existing item.
#[derive(Debug, Deserialize)]
pub struct UpdateItemRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_item_with_valid_fields() {
        let item = Item::new("Test item", "user-1").unwrap();
        assert_eq!(item.name(), "Test item");
        assert_eq!(item.owner_id(), "user-1");
        assert_eq!(item.status(), ItemStatus::Active);
        assert!(item.description().is_none());
    }

    #[test]
    fn create_item_rejects_empty_name() {
        let result = Item::new("", "user-1");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("name must not be empty"));
    }

    #[test]
    fn create_item_rejects_whitespace_name() {
        let result = Item::new("   ", "user-1");
        assert!(result.is_err());
    }

    #[test]
    fn create_item_rejects_empty_owner() {
        let result = Item::new("Valid name", "");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("owner_id must not be empty"));
    }

    #[test]
    fn set_description() {
        let mut item = Item::new("Test", "user-1").unwrap();
        item.set_description("A description");
        assert_eq!(item.description(), Some("A description"));
    }

    #[test]
    fn archive_active_item() {
        let mut item = Item::new("Test", "user-1").unwrap();
        item.archive().unwrap();
        assert_eq!(item.status(), ItemStatus::Archived);
    }

    #[test]
    fn archive_deleted_item_fails() {
        let mut item = Item::new("Test", "user-1").unwrap();
        item.delete();
        let result = item.archive();
        assert!(result.is_err());
    }

    #[test]
    fn delete_item() {
        let mut item = Item::new("Test", "user-1").unwrap();
        item.delete();
        assert_eq!(item.status(), ItemStatus::Deleted);
    }

    #[test]
    fn item_serializes_to_json() {
        let item = Item::new("Test", "user-1").unwrap();
        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("\"name\":\"Test\""));
        assert!(json.contains("\"status\":\"active\""));
    }

    #[test]
    fn item_roundtrips_through_json() {
        let original = Item::new("Roundtrip", "user-1").unwrap();
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Item = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn default_status_is_active() {
        assert_eq!(ItemStatus::default(), ItemStatus::Active);
    }
}
