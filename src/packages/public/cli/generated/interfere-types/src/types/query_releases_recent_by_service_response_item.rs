pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryReleasesRecentByServiceResponseItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub slug: String,
    #[serde(rename = "surfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_id: Option<String>,
    #[serde(rename = "serviceId")]
    #[serde(default)]
    pub service_id: String,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(rename = "buildId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_id: Option<String>,
    #[serde(rename = "sourceMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_metadata: Option<QueryReleasesRecentByServiceResponseItemSourceMetadata>,
    #[serde(rename = "destinationMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_metadata: Option<QueryReleasesRecentByServiceResponseItemDestinationMetadata>,
    #[serde(rename = "producerVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer_version: Option<String>,
    #[serde(rename = "sourceChangeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_change_id: Option<String>,
    #[serde(rename = "destinationChangeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_change_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    pub status: QueryReleasesRecentByServiceResponseItemStatus,
    #[serde(rename = "sourceFileCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file_count: Option<i64>,
    #[serde(rename = "sourceMapCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_map_count: Option<i64>,
    #[serde(rename = "deployedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub deployed_at: Option<f64>,
    #[serde(rename = "revokedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub revoked_at: Option<f64>,
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
    #[serde(rename = "sourceChange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_change: Option<QueryReleasesRecentByServiceResponseItemSourceChange>,
    #[serde(rename = "destinationChange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_change: Option<QueryReleasesRecentByServiceResponseItemDestinationChange>,
    #[serde(default)]
    pub deployments: Vec<QueryReleasesRecentByServiceResponseItemDeploymentsItem>,
}

impl QueryReleasesRecentByServiceResponseItem {
    pub fn builder() -> QueryReleasesRecentByServiceResponseItemBuilder {
        <QueryReleasesRecentByServiceResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryReleasesRecentByServiceResponseItemBuilder {
    id: Option<String>,
    slug: Option<String>,
    surface_id: Option<String>,
    service_id: Option<String>,
    organization_id: Option<String>,
    build_id: Option<String>,
    source_metadata: Option<QueryReleasesRecentByServiceResponseItemSourceMetadata>,
    destination_metadata: Option<QueryReleasesRecentByServiceResponseItemDestinationMetadata>,
    producer_version: Option<String>,
    source_change_id: Option<String>,
    destination_change_id: Option<String>,
    environment: Option<String>,
    status: Option<QueryReleasesRecentByServiceResponseItemStatus>,
    source_file_count: Option<i64>,
    source_map_count: Option<i64>,
    deployed_at: Option<f64>,
    revoked_at: Option<f64>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
    source_change: Option<QueryReleasesRecentByServiceResponseItemSourceChange>,
    destination_change: Option<QueryReleasesRecentByServiceResponseItemDestinationChange>,
    deployments: Option<Vec<QueryReleasesRecentByServiceResponseItemDeploymentsItem>>,
}

impl QueryReleasesRecentByServiceResponseItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn surface_id(mut self, value: impl Into<String>) -> Self {
        self.surface_id = Some(value.into());
        self
    }

    pub fn service_id(mut self, value: impl Into<String>) -> Self {
        self.service_id = Some(value.into());
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn build_id(mut self, value: impl Into<String>) -> Self {
        self.build_id = Some(value.into());
        self
    }

    pub fn source_metadata(mut self, value: QueryReleasesRecentByServiceResponseItemSourceMetadata) -> Self {
        self.source_metadata = Some(value);
        self
    }

    pub fn destination_metadata(mut self, value: QueryReleasesRecentByServiceResponseItemDestinationMetadata) -> Self {
        self.destination_metadata = Some(value);
        self
    }

    pub fn producer_version(mut self, value: impl Into<String>) -> Self {
        self.producer_version = Some(value.into());
        self
    }

    pub fn source_change_id(mut self, value: impl Into<String>) -> Self {
        self.source_change_id = Some(value.into());
        self
    }

    pub fn destination_change_id(mut self, value: impl Into<String>) -> Self {
        self.destination_change_id = Some(value.into());
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn status(mut self, value: QueryReleasesRecentByServiceResponseItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn source_file_count(mut self, value: i64) -> Self {
        self.source_file_count = Some(value);
        self
    }

    pub fn source_map_count(mut self, value: i64) -> Self {
        self.source_map_count = Some(value);
        self
    }

    pub fn deployed_at(mut self, value: f64) -> Self {
        self.deployed_at = Some(value);
        self
    }

    pub fn revoked_at(mut self, value: f64) -> Self {
        self.revoked_at = Some(value);
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

    pub fn source_change(mut self, value: QueryReleasesRecentByServiceResponseItemSourceChange) -> Self {
        self.source_change = Some(value);
        self
    }

    pub fn destination_change(mut self, value: QueryReleasesRecentByServiceResponseItemDestinationChange) -> Self {
        self.destination_change = Some(value);
        self
    }

    pub fn deployments(mut self, value: Vec<QueryReleasesRecentByServiceResponseItemDeploymentsItem>) -> Self {
        self.deployments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryReleasesRecentByServiceResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QueryReleasesRecentByServiceResponseItemBuilder::id)
    /// - [`slug`](QueryReleasesRecentByServiceResponseItemBuilder::slug)
    /// - [`service_id`](QueryReleasesRecentByServiceResponseItemBuilder::service_id)
    /// - [`organization_id`](QueryReleasesRecentByServiceResponseItemBuilder::organization_id)
    /// - [`status`](QueryReleasesRecentByServiceResponseItemBuilder::status)
    /// - [`created_at`](QueryReleasesRecentByServiceResponseItemBuilder::created_at)
    /// - [`updated_at`](QueryReleasesRecentByServiceResponseItemBuilder::updated_at)
    /// - [`deployments`](QueryReleasesRecentByServiceResponseItemBuilder::deployments)
    pub fn build(self) -> Result<QueryReleasesRecentByServiceResponseItem, BuildError> {
        Ok(QueryReleasesRecentByServiceResponseItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            surface_id: self.surface_id,
            service_id: self.service_id.ok_or_else(|| BuildError::missing_field("service_id"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            build_id: self.build_id,
            source_metadata: self.source_metadata,
            destination_metadata: self.destination_metadata,
            producer_version: self.producer_version,
            source_change_id: self.source_change_id,
            destination_change_id: self.destination_change_id,
            environment: self.environment,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            source_file_count: self.source_file_count,
            source_map_count: self.source_map_count,
            deployed_at: self.deployed_at,
            revoked_at: self.revoked_at,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            source_change: self.source_change,
            destination_change: self.destination_change,
            deployments: self.deployments.ok_or_else(|| BuildError::missing_field("deployments"))?,
        })
    }
}
