pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryServicesForWorkspaceResponseItemSourceDatasource {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "surfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_id: Option<String>,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub availability: String,
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

impl QueryServicesForWorkspaceResponseItemSourceDatasource {
    pub fn builder() -> QueryServicesForWorkspaceResponseItemSourceDatasourceBuilder {
        <QueryServicesForWorkspaceResponseItemSourceDatasourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryServicesForWorkspaceResponseItemSourceDatasourceBuilder {
    id: Option<String>,
    organization_id: Option<String>,
    connection_id: Option<String>,
    surface_id: Option<String>,
    kind: Option<String>,
    name: Option<String>,
    url: Option<String>,
    availability: Option<String>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
}

impl QueryServicesForWorkspaceResponseItemSourceDatasourceBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn connection_id(mut self, value: impl Into<String>) -> Self {
        self.connection_id = Some(value.into());
        self
    }

    pub fn surface_id(mut self, value: impl Into<String>) -> Self {
        self.surface_id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn availability(mut self, value: impl Into<String>) -> Self {
        self.availability = Some(value.into());
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

    /// Consumes the builder and constructs a [`QueryServicesForWorkspaceResponseItemSourceDatasource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QueryServicesForWorkspaceResponseItemSourceDatasourceBuilder::id)
    /// - [`organization_id`](QueryServicesForWorkspaceResponseItemSourceDatasourceBuilder::organization_id)
    /// - [`connection_id`](QueryServicesForWorkspaceResponseItemSourceDatasourceBuilder::connection_id)
    /// - [`kind`](QueryServicesForWorkspaceResponseItemSourceDatasourceBuilder::kind)
    /// - [`name`](QueryServicesForWorkspaceResponseItemSourceDatasourceBuilder::name)
    /// - [`url`](QueryServicesForWorkspaceResponseItemSourceDatasourceBuilder::url)
    /// - [`availability`](QueryServicesForWorkspaceResponseItemSourceDatasourceBuilder::availability)
    /// - [`created_at`](QueryServicesForWorkspaceResponseItemSourceDatasourceBuilder::created_at)
    /// - [`updated_at`](QueryServicesForWorkspaceResponseItemSourceDatasourceBuilder::updated_at)
    pub fn build(self) -> Result<QueryServicesForWorkspaceResponseItemSourceDatasource, BuildError> {
        Ok(QueryServicesForWorkspaceResponseItemSourceDatasource {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            connection_id: self.connection_id.ok_or_else(|| BuildError::missing_field("connection_id"))?,
            surface_id: self.surface_id,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            availability: self.availability.ok_or_else(|| BuildError::missing_field("availability"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
