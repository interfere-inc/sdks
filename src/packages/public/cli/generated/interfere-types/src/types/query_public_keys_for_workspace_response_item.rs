pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryPublicKeysForWorkspaceResponseItem {
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
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "lastUsedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub last_used_at: Option<f64>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(rename = "surfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_id: Option<String>,
}

impl QueryPublicKeysForWorkspaceResponseItem {
    pub fn builder() -> QueryPublicKeysForWorkspaceResponseItemBuilder {
        <QueryPublicKeysForWorkspaceResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryPublicKeysForWorkspaceResponseItemBuilder {
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
    content: Option<String>,
    id: Option<String>,
    last_used_at: Option<f64>,
    name: Option<String>,
    organization_id: Option<String>,
    surface_id: Option<String>,
}

impl QueryPublicKeysForWorkspaceResponseItemBuilder {
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

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_used_at(mut self, value: f64) -> Self {
        self.last_used_at = Some(value);
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

    /// Consumes the builder and constructs a [`QueryPublicKeysForWorkspaceResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](QueryPublicKeysForWorkspaceResponseItemBuilder::created_at)
    /// - [`updated_at`](QueryPublicKeysForWorkspaceResponseItemBuilder::updated_at)
    /// - [`content`](QueryPublicKeysForWorkspaceResponseItemBuilder::content)
    /// - [`id`](QueryPublicKeysForWorkspaceResponseItemBuilder::id)
    /// - [`name`](QueryPublicKeysForWorkspaceResponseItemBuilder::name)
    /// - [`organization_id`](QueryPublicKeysForWorkspaceResponseItemBuilder::organization_id)
    pub fn build(self) -> Result<QueryPublicKeysForWorkspaceResponseItem, BuildError> {
        Ok(QueryPublicKeysForWorkspaceResponseItem {
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_used_at: self.last_used_at,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            surface_id: self.surface_id,
        })
    }
}
