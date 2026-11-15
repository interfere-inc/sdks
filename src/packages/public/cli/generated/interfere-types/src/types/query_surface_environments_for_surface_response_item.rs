pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QuerySurfaceEnvironmentsForSurfaceResponseItem {
    #[serde(rename = "displayName")]
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "isProduction")]
    #[serde(default)]
    pub is_production: bool,
    #[serde(default)]
    pub reportable: bool,
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
    #[serde(default)]
    pub identifiers: Vec<QuerySurfaceEnvironmentsForSurfaceResponseItemIdentifiersItem>,
}

impl QuerySurfaceEnvironmentsForSurfaceResponseItem {
    pub fn builder() -> QuerySurfaceEnvironmentsForSurfaceResponseItemBuilder {
        <QuerySurfaceEnvironmentsForSurfaceResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuerySurfaceEnvironmentsForSurfaceResponseItemBuilder {
    display_name: Option<String>,
    id: Option<String>,
    is_production: Option<bool>,
    reportable: Option<bool>,
    surface_id: Option<String>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
    identifiers: Option<Vec<QuerySurfaceEnvironmentsForSurfaceResponseItemIdentifiersItem>>,
}

impl QuerySurfaceEnvironmentsForSurfaceResponseItemBuilder {
    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_production(mut self, value: bool) -> Self {
        self.is_production = Some(value);
        self
    }

    pub fn reportable(mut self, value: bool) -> Self {
        self.reportable = Some(value);
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

    pub fn identifiers(mut self, value: Vec<QuerySurfaceEnvironmentsForSurfaceResponseItemIdentifiersItem>) -> Self {
        self.identifiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QuerySurfaceEnvironmentsForSurfaceResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display_name`](QuerySurfaceEnvironmentsForSurfaceResponseItemBuilder::display_name)
    /// - [`id`](QuerySurfaceEnvironmentsForSurfaceResponseItemBuilder::id)
    /// - [`is_production`](QuerySurfaceEnvironmentsForSurfaceResponseItemBuilder::is_production)
    /// - [`reportable`](QuerySurfaceEnvironmentsForSurfaceResponseItemBuilder::reportable)
    /// - [`surface_id`](QuerySurfaceEnvironmentsForSurfaceResponseItemBuilder::surface_id)
    /// - [`created_at`](QuerySurfaceEnvironmentsForSurfaceResponseItemBuilder::created_at)
    /// - [`updated_at`](QuerySurfaceEnvironmentsForSurfaceResponseItemBuilder::updated_at)
    /// - [`identifiers`](QuerySurfaceEnvironmentsForSurfaceResponseItemBuilder::identifiers)
    pub fn build(self) -> Result<QuerySurfaceEnvironmentsForSurfaceResponseItem, BuildError> {
        Ok(QuerySurfaceEnvironmentsForSurfaceResponseItem {
            display_name: self.display_name.ok_or_else(|| BuildError::missing_field("display_name"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_production: self.is_production.ok_or_else(|| BuildError::missing_field("is_production"))?,
            reportable: self.reportable.ok_or_else(|| BuildError::missing_field("reportable"))?,
            surface_id: self.surface_id.ok_or_else(|| BuildError::missing_field("surface_id"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            identifiers: self.identifiers.ok_or_else(|| BuildError::missing_field("identifiers"))?,
        })
    }
}
