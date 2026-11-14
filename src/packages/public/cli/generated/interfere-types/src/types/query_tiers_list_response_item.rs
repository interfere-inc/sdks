pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryTiersListResponseItem {
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(default)]
    pub value: String,
    pub shade: QueryTiersListResponseItemShade,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub created_at: f64,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub updated_at: f64,
}

impl QueryTiersListResponseItem {
    pub fn builder() -> QueryTiersListResponseItemBuilder {
        <QueryTiersListResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryTiersListResponseItemBuilder {
    organization_id: Option<String>,
    value: Option<String>,
    shade: Option<QueryTiersListResponseItemShade>,
    created_at: Option<f64>,
    updated_at: Option<f64>,
}

impl QueryTiersListResponseItemBuilder {
    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn shade(mut self, value: QueryTiersListResponseItemShade) -> Self {
        self.shade = Some(value);
        self
    }

    pub fn created_at(mut self, value: f64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: f64) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryTiersListResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`organization_id`](QueryTiersListResponseItemBuilder::organization_id)
    /// - [`value`](QueryTiersListResponseItemBuilder::value)
    /// - [`shade`](QueryTiersListResponseItemBuilder::shade)
    /// - [`created_at`](QueryTiersListResponseItemBuilder::created_at)
    /// - [`updated_at`](QueryTiersListResponseItemBuilder::updated_at)
    pub fn build(self) -> Result<QueryTiersListResponseItem, BuildError> {
        Ok(QueryTiersListResponseItem {
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
            shade: self.shade.ok_or_else(|| BuildError::missing_field("shade"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
