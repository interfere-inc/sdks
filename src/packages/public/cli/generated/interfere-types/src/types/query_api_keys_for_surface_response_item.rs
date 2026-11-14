pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryApiKeysForSurfaceResponseItem {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub created_at: f64,
    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub deleted_at: Option<f64>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub updated_at: f64,
    #[serde(rename = "clerkApiKeyId")]
    #[serde(default)]
    pub clerk_api_key_id: String,
    #[serde(rename = "createdByUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_user_id: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "keyHint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_hint: Option<String>,
    #[serde(rename = "lastUsedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub last_used_at: Option<f64>,
    #[serde(rename = "managedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub managed_at: Option<f64>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "organizationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    #[serde(rename = "surfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_id: Option<String>,
}

impl QueryApiKeysForSurfaceResponseItem {
    pub fn builder() -> QueryApiKeysForSurfaceResponseItemBuilder {
        <QueryApiKeysForSurfaceResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryApiKeysForSurfaceResponseItemBuilder {
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
    clerk_api_key_id: Option<String>,
    created_by_user_id: Option<String>,
    id: Option<String>,
    key_hint: Option<String>,
    last_used_at: Option<f64>,
    managed_at: Option<f64>,
    name: Option<String>,
    organization_id: Option<String>,
    surface_id: Option<String>,
}

impl QueryApiKeysForSurfaceResponseItemBuilder {
    pub fn created_at(mut self, value: f64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: f64) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: f64) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn clerk_api_key_id(mut self, value: impl Into<String>) -> Self {
        self.clerk_api_key_id = Some(value.into());
        self
    }

    pub fn created_by_user_id(mut self, value: impl Into<String>) -> Self {
        self.created_by_user_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn key_hint(mut self, value: impl Into<String>) -> Self {
        self.key_hint = Some(value.into());
        self
    }

    pub fn last_used_at(mut self, value: f64) -> Self {
        self.last_used_at = Some(value);
        self
    }

    pub fn managed_at(mut self, value: f64) -> Self {
        self.managed_at = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn surface_id(mut self, value: impl Into<String>) -> Self {
        self.surface_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryApiKeysForSurfaceResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](QueryApiKeysForSurfaceResponseItemBuilder::created_at)
    /// - [`updated_at`](QueryApiKeysForSurfaceResponseItemBuilder::updated_at)
    /// - [`clerk_api_key_id`](QueryApiKeysForSurfaceResponseItemBuilder::clerk_api_key_id)
    /// - [`id`](QueryApiKeysForSurfaceResponseItemBuilder::id)
    /// - [`name`](QueryApiKeysForSurfaceResponseItemBuilder::name)
    pub fn build(self) -> Result<QueryApiKeysForSurfaceResponseItem, BuildError> {
        Ok(QueryApiKeysForSurfaceResponseItem {
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            clerk_api_key_id: self.clerk_api_key_id.ok_or_else(|| BuildError::missing_field("clerk_api_key_id"))?,
            created_by_user_id: self.created_by_user_id,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            key_hint: self.key_hint,
            last_used_at: self.last_used_at,
            managed_at: self.managed_at,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            organization_id: self.organization_id,
            surface_id: self.surface_id,
        })
    }
}
