pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryServicesByIdsResponseItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "connectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "sourceDatasourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_datasource_id: Option<String>,
    #[serde(rename = "sourceWorkingDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_working_directory: Option<String>,
    #[serde(rename = "reportableEnvironment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reportable_environment: Option<String>,
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "typeSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_source: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub created_at: f64,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub updated_at: f64,
    #[serde(rename = "lastSeenAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub last_seen_at: f64,
    #[serde(rename = "firstTelemetryAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub first_telemetry_at: Option<f64>,
    #[serde(rename = "websiteSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_settings: Option<QueryServicesByIdsResponseItemWebsiteSettings>,
    #[serde(rename = "destinationResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_resource: Option<String>,
    #[serde(rename = "pausedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub paused_at: Option<f64>,
    #[serde(rename = "mergedIntoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_into_id: Option<String>,
    #[serde(rename = "archivedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub archived_at: Option<f64>,
    #[serde(rename = "sourceDatasource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_datasource: Option<QueryServicesByIdsResponseItemSourceDatasource>,
}

impl QueryServicesByIdsResponseItem {
    pub fn builder() -> QueryServicesByIdsResponseItemBuilder {
        <QueryServicesByIdsResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryServicesByIdsResponseItemBuilder {
    id: Option<String>,
    organization_id: Option<String>,
    namespace: Option<String>,
    name: Option<String>,
    connection_id: Option<String>,
    resource: Option<String>,
    source_datasource_id: Option<String>,
    source_working_directory: Option<String>,
    reportable_environment: Option<String>,
    r#type: Option<String>,
    type_source: Option<String>,
    created_at: Option<f64>,
    updated_at: Option<f64>,
    last_seen_at: Option<f64>,
    first_telemetry_at: Option<f64>,
    website_settings: Option<QueryServicesByIdsResponseItemWebsiteSettings>,
    destination_resource: Option<String>,
    paused_at: Option<f64>,
    merged_into_id: Option<String>,
    archived_at: Option<f64>,
    source_datasource: Option<QueryServicesByIdsResponseItemSourceDatasource>,
}

impl QueryServicesByIdsResponseItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn namespace(mut self, value: impl Into<String>) -> Self {
        self.namespace = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn connection_id(mut self, value: impl Into<String>) -> Self {
        self.connection_id = Some(value.into());
        self
    }

    pub fn resource(mut self, value: impl Into<String>) -> Self {
        self.resource = Some(value.into());
        self
    }

    pub fn source_datasource_id(mut self, value: impl Into<String>) -> Self {
        self.source_datasource_id = Some(value.into());
        self
    }

    pub fn source_working_directory(mut self, value: impl Into<String>) -> Self {
        self.source_working_directory = Some(value.into());
        self
    }

    pub fn reportable_environment(mut self, value: impl Into<String>) -> Self {
        self.reportable_environment = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn type_source(mut self, value: impl Into<String>) -> Self {
        self.type_source = Some(value.into());
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

    pub fn last_seen_at(mut self, value: f64) -> Self {
        self.last_seen_at = Some(value);
        self
    }

    pub fn first_telemetry_at(mut self, value: f64) -> Self {
        self.first_telemetry_at = Some(value);
        self
    }

    pub fn website_settings(mut self, value: QueryServicesByIdsResponseItemWebsiteSettings) -> Self {
        self.website_settings = Some(value);
        self
    }

    pub fn destination_resource(mut self, value: impl Into<String>) -> Self {
        self.destination_resource = Some(value.into());
        self
    }

    pub fn paused_at(mut self, value: f64) -> Self {
        self.paused_at = Some(value);
        self
    }

    pub fn merged_into_id(mut self, value: impl Into<String>) -> Self {
        self.merged_into_id = Some(value.into());
        self
    }

    pub fn archived_at(mut self, value: f64) -> Self {
        self.archived_at = Some(value);
        self
    }

    pub fn source_datasource(mut self, value: QueryServicesByIdsResponseItemSourceDatasource) -> Self {
        self.source_datasource = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryServicesByIdsResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QueryServicesByIdsResponseItemBuilder::id)
    /// - [`organization_id`](QueryServicesByIdsResponseItemBuilder::organization_id)
    /// - [`name`](QueryServicesByIdsResponseItemBuilder::name)
    /// - [`r#type`](QueryServicesByIdsResponseItemBuilder::r#type)
    /// - [`created_at`](QueryServicesByIdsResponseItemBuilder::created_at)
    /// - [`updated_at`](QueryServicesByIdsResponseItemBuilder::updated_at)
    /// - [`last_seen_at`](QueryServicesByIdsResponseItemBuilder::last_seen_at)
    pub fn build(self) -> Result<QueryServicesByIdsResponseItem, BuildError> {
        Ok(QueryServicesByIdsResponseItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            namespace: self.namespace,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            connection_id: self.connection_id,
            resource: self.resource,
            source_datasource_id: self.source_datasource_id,
            source_working_directory: self.source_working_directory,
            reportable_environment: self.reportable_environment,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            type_source: self.type_source,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            last_seen_at: self.last_seen_at.ok_or_else(|| BuildError::missing_field("last_seen_at"))?,
            first_telemetry_at: self.first_telemetry_at,
            website_settings: self.website_settings,
            destination_resource: self.destination_resource,
            paused_at: self.paused_at,
            merged_into_id: self.merged_into_id,
            archived_at: self.archived_at,
            source_datasource: self.source_datasource,
        })
    }
}
