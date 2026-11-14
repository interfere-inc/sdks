pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QuerySurfaceEnvironmentsForSurfaceResponseItemIdentifiersItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "surfaceEnvironmentId")]
    #[serde(default)]
    pub surface_environment_id: String,
    #[serde(rename = "surfaceId")]
    #[serde(default)]
    pub surface_id: String,
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
}

impl QuerySurfaceEnvironmentsForSurfaceResponseItemIdentifiersItem {
    pub fn builder() -> QuerySurfaceEnvironmentsForSurfaceResponseItemIdentifiersItemBuilder {
        <QuerySurfaceEnvironmentsForSurfaceResponseItemIdentifiersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuerySurfaceEnvironmentsForSurfaceResponseItemIdentifiersItemBuilder {
    id: Option<String>,
    identifier: Option<String>,
    surface_environment_id: Option<String>,
    surface_id: Option<String>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
}

impl QuerySurfaceEnvironmentsForSurfaceResponseItemIdentifiersItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn identifier(mut self, value: impl Into<String>) -> Self {
        self.identifier = Some(value.into());
        self
    }

    pub fn surface_environment_id(mut self, value: impl Into<String>) -> Self {
        self.surface_environment_id = Some(value.into());
        self
    }

    pub fn surface_id(mut self, value: impl Into<String>) -> Self {
        self.surface_id = Some(value.into());
        self
    }

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

    /// Consumes the builder and constructs a [`QuerySurfaceEnvironmentsForSurfaceResponseItemIdentifiersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QuerySurfaceEnvironmentsForSurfaceResponseItemIdentifiersItemBuilder::id)
    /// - [`identifier`](QuerySurfaceEnvironmentsForSurfaceResponseItemIdentifiersItemBuilder::identifier)
    /// - [`surface_environment_id`](QuerySurfaceEnvironmentsForSurfaceResponseItemIdentifiersItemBuilder::surface_environment_id)
    /// - [`surface_id`](QuerySurfaceEnvironmentsForSurfaceResponseItemIdentifiersItemBuilder::surface_id)
    /// - [`created_at`](QuerySurfaceEnvironmentsForSurfaceResponseItemIdentifiersItemBuilder::created_at)
    /// - [`updated_at`](QuerySurfaceEnvironmentsForSurfaceResponseItemIdentifiersItemBuilder::updated_at)
    pub fn build(self) -> Result<QuerySurfaceEnvironmentsForSurfaceResponseItemIdentifiersItem, BuildError> {
        Ok(QuerySurfaceEnvironmentsForSurfaceResponseItemIdentifiersItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            identifier: self.identifier.ok_or_else(|| BuildError::missing_field("identifier"))?,
            surface_environment_id: self.surface_environment_id.ok_or_else(|| BuildError::missing_field("surface_environment_id"))?,
            surface_id: self.surface_id.ok_or_else(|| BuildError::missing_field("surface_id"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
