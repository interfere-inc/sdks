pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryEnvironmentsListResponseItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "isProduction")]
    #[serde(default)]
    pub is_production: bool,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(default)]
    pub reportable: bool,
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

impl QueryEnvironmentsListResponseItem {
    pub fn builder() -> QueryEnvironmentsListResponseItemBuilder {
        <QueryEnvironmentsListResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryEnvironmentsListResponseItemBuilder {
    id: Option<String>,
    is_production: Option<bool>,
    name: Option<String>,
    organization_id: Option<String>,
    reportable: Option<bool>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
}

impl QueryEnvironmentsListResponseItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_production(mut self, value: bool) -> Self {
        self.is_production = Some(value);
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

    pub fn reportable(mut self, value: bool) -> Self {
        self.reportable = Some(value);
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

    /// Consumes the builder and constructs a [`QueryEnvironmentsListResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QueryEnvironmentsListResponseItemBuilder::id)
    /// - [`is_production`](QueryEnvironmentsListResponseItemBuilder::is_production)
    /// - [`name`](QueryEnvironmentsListResponseItemBuilder::name)
    /// - [`organization_id`](QueryEnvironmentsListResponseItemBuilder::organization_id)
    /// - [`reportable`](QueryEnvironmentsListResponseItemBuilder::reportable)
    /// - [`created_at`](QueryEnvironmentsListResponseItemBuilder::created_at)
    /// - [`updated_at`](QueryEnvironmentsListResponseItemBuilder::updated_at)
    pub fn build(self) -> Result<QueryEnvironmentsListResponseItem, BuildError> {
        Ok(QueryEnvironmentsListResponseItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_production: self.is_production.ok_or_else(|| BuildError::missing_field("is_production"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            reportable: self.reportable.ok_or_else(|| BuildError::missing_field("reportable"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
