pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryReleasesRecentBySurfaceResponseItemDeploymentsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(rename = "surfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_id: Option<String>,
    #[serde(rename = "releaseId")]
    #[serde(default)]
    pub release_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(default)]
    pub provider: String,
    #[serde(rename = "surfaceChangeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_change_id: Option<String>,
    pub status: QueryReleasesRecentBySurfaceResponseItemDeploymentsItemStatus,
    #[serde(rename = "deployedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub deployed_at: Option<f64>,
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

impl QueryReleasesRecentBySurfaceResponseItemDeploymentsItem {
    pub fn builder() -> QueryReleasesRecentBySurfaceResponseItemDeploymentsItemBuilder {
        <QueryReleasesRecentBySurfaceResponseItemDeploymentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryReleasesRecentBySurfaceResponseItemDeploymentsItemBuilder {
    id: Option<String>,
    organization_id: Option<String>,
    surface_id: Option<String>,
    release_id: Option<String>,
    environment: Option<String>,
    provider: Option<String>,
    surface_change_id: Option<String>,
    status: Option<QueryReleasesRecentBySurfaceResponseItemDeploymentsItemStatus>,
    deployed_at: Option<f64>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
}

impl QueryReleasesRecentBySurfaceResponseItemDeploymentsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn release_id(mut self, value: impl Into<String>) -> Self {
        self.release_id = Some(value.into());
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn surface_change_id(mut self, value: impl Into<String>) -> Self {
        self.surface_change_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: QueryReleasesRecentBySurfaceResponseItemDeploymentsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn deployed_at(mut self, value: f64) -> Self {
        self.deployed_at = Some(value);
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

    /// Consumes the builder and constructs a [`QueryReleasesRecentBySurfaceResponseItemDeploymentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QueryReleasesRecentBySurfaceResponseItemDeploymentsItemBuilder::id)
    /// - [`organization_id`](QueryReleasesRecentBySurfaceResponseItemDeploymentsItemBuilder::organization_id)
    /// - [`release_id`](QueryReleasesRecentBySurfaceResponseItemDeploymentsItemBuilder::release_id)
    /// - [`provider`](QueryReleasesRecentBySurfaceResponseItemDeploymentsItemBuilder::provider)
    /// - [`status`](QueryReleasesRecentBySurfaceResponseItemDeploymentsItemBuilder::status)
    /// - [`created_at`](QueryReleasesRecentBySurfaceResponseItemDeploymentsItemBuilder::created_at)
    /// - [`updated_at`](QueryReleasesRecentBySurfaceResponseItemDeploymentsItemBuilder::updated_at)
    pub fn build(self) -> Result<QueryReleasesRecentBySurfaceResponseItemDeploymentsItem, BuildError> {
        Ok(QueryReleasesRecentBySurfaceResponseItemDeploymentsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            surface_id: self.surface_id,
            release_id: self.release_id.ok_or_else(|| BuildError::missing_field("release_id"))?,
            environment: self.environment,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            surface_change_id: self.surface_change_id,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            deployed_at: self.deployed_at,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
