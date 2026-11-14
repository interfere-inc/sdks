pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuerySurfacesListResponseItem {
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
    #[serde(rename = "anonymousReplayPolicy")]
    #[serde(default)]
    pub anonymous_replay_policy: String,
    #[serde(rename = "anonymousUserTracking")]
    #[serde(default)]
    pub anonymous_user_tracking: bool,
    #[serde(rename = "destinationConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_connection_id: Option<String>,
    #[serde(rename = "destinationMappingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_mapping_id: Option<String>,
    #[serde(rename = "destinationMappingLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_mapping_label: Option<String>,
    #[serde(rename = "destinationMappingUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_mapping_url: Option<String>,
    #[serde(rename = "environmentFiltering")]
    #[serde(default)]
    pub environment_filtering: bool,
    #[serde(rename = "explicitOnly")]
    #[serde(default)]
    pub explicit_only: bool,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "onboardedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub onboarded_at: Option<f64>,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(rename = "primaryEnvironmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_environment_id: Option<String>,
    #[serde(rename = "primaryEnvironmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_environment_name: Option<String>,
    #[serde(rename = "sdkConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdk_config: Option<QuerySurfacesListResponseItemSdkConfig>,
    #[serde(default)]
    pub slug: String,
    #[serde(rename = "sourceDatasourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_datasource_id: Option<String>,
    #[serde(rename = "sourceConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_connection_id: Option<String>,
    #[serde(rename = "sourceMappingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_mapping_id: Option<String>,
    #[serde(rename = "sourceMappingLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_mapping_label: Option<String>,
    #[serde(rename = "sourceMappingUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_mapping_url: Option<String>,
    #[serde(rename = "sourceWorkingDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_working_directory: Option<String>,
    pub r#type: QuerySurfacesListResponseItemType,
}

impl QuerySurfacesListResponseItem {
    pub fn builder() -> QuerySurfacesListResponseItemBuilder {
        <QuerySurfacesListResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuerySurfacesListResponseItemBuilder {
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
    anonymous_replay_policy: Option<String>,
    anonymous_user_tracking: Option<bool>,
    destination_connection_id: Option<String>,
    destination_mapping_id: Option<String>,
    destination_mapping_label: Option<String>,
    destination_mapping_url: Option<String>,
    environment_filtering: Option<bool>,
    explicit_only: Option<bool>,
    id: Option<String>,
    name: Option<String>,
    onboarded_at: Option<f64>,
    organization_id: Option<String>,
    primary_environment_id: Option<String>,
    primary_environment_name: Option<String>,
    sdk_config: Option<QuerySurfacesListResponseItemSdkConfig>,
    slug: Option<String>,
    source_datasource_id: Option<String>,
    source_connection_id: Option<String>,
    source_mapping_id: Option<String>,
    source_mapping_label: Option<String>,
    source_mapping_url: Option<String>,
    source_working_directory: Option<String>,
    r#type: Option<QuerySurfacesListResponseItemType>,
}

impl QuerySurfacesListResponseItemBuilder {
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

    pub fn anonymous_replay_policy(mut self, value: impl Into<String>) -> Self {
        self.anonymous_replay_policy = Some(value.into());
        self
    }

    pub fn anonymous_user_tracking(mut self, value: bool) -> Self {
        self.anonymous_user_tracking = Some(value);
        self
    }

    pub fn destination_connection_id(mut self, value: impl Into<String>) -> Self {
        self.destination_connection_id = Some(value.into());
        self
    }

    pub fn destination_mapping_id(mut self, value: impl Into<String>) -> Self {
        self.destination_mapping_id = Some(value.into());
        self
    }

    pub fn destination_mapping_label(mut self, value: impl Into<String>) -> Self {
        self.destination_mapping_label = Some(value.into());
        self
    }

    pub fn destination_mapping_url(mut self, value: impl Into<String>) -> Self {
        self.destination_mapping_url = Some(value.into());
        self
    }

    pub fn environment_filtering(mut self, value: bool) -> Self {
        self.environment_filtering = Some(value);
        self
    }

    pub fn explicit_only(mut self, value: bool) -> Self {
        self.explicit_only = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn onboarded_at(mut self, value: f64) -> Self {
        self.onboarded_at = Some(value);
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn primary_environment_id(mut self, value: impl Into<String>) -> Self {
        self.primary_environment_id = Some(value.into());
        self
    }

    pub fn primary_environment_name(mut self, value: impl Into<String>) -> Self {
        self.primary_environment_name = Some(value.into());
        self
    }

    pub fn sdk_config(mut self, value: QuerySurfacesListResponseItemSdkConfig) -> Self {
        self.sdk_config = Some(value);
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn source_datasource_id(mut self, value: impl Into<String>) -> Self {
        self.source_datasource_id = Some(value.into());
        self
    }

    pub fn source_connection_id(mut self, value: impl Into<String>) -> Self {
        self.source_connection_id = Some(value.into());
        self
    }

    pub fn source_mapping_id(mut self, value: impl Into<String>) -> Self {
        self.source_mapping_id = Some(value.into());
        self
    }

    pub fn source_mapping_label(mut self, value: impl Into<String>) -> Self {
        self.source_mapping_label = Some(value.into());
        self
    }

    pub fn source_mapping_url(mut self, value: impl Into<String>) -> Self {
        self.source_mapping_url = Some(value.into());
        self
    }

    pub fn source_working_directory(mut self, value: impl Into<String>) -> Self {
        self.source_working_directory = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: QuerySurfacesListResponseItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QuerySurfacesListResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](QuerySurfacesListResponseItemBuilder::created_at)
    /// - [`updated_at`](QuerySurfacesListResponseItemBuilder::updated_at)
    /// - [`anonymous_replay_policy`](QuerySurfacesListResponseItemBuilder::anonymous_replay_policy)
    /// - [`anonymous_user_tracking`](QuerySurfacesListResponseItemBuilder::anonymous_user_tracking)
    /// - [`environment_filtering`](QuerySurfacesListResponseItemBuilder::environment_filtering)
    /// - [`explicit_only`](QuerySurfacesListResponseItemBuilder::explicit_only)
    /// - [`id`](QuerySurfacesListResponseItemBuilder::id)
    /// - [`name`](QuerySurfacesListResponseItemBuilder::name)
    /// - [`organization_id`](QuerySurfacesListResponseItemBuilder::organization_id)
    /// - [`slug`](QuerySurfacesListResponseItemBuilder::slug)
    /// - [`r#type`](QuerySurfacesListResponseItemBuilder::r#type)
    pub fn build(self) -> Result<QuerySurfacesListResponseItem, BuildError> {
        Ok(QuerySurfacesListResponseItem {
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            anonymous_replay_policy: self.anonymous_replay_policy.ok_or_else(|| BuildError::missing_field("anonymous_replay_policy"))?,
            anonymous_user_tracking: self.anonymous_user_tracking.ok_or_else(|| BuildError::missing_field("anonymous_user_tracking"))?,
            destination_connection_id: self.destination_connection_id,
            destination_mapping_id: self.destination_mapping_id,
            destination_mapping_label: self.destination_mapping_label,
            destination_mapping_url: self.destination_mapping_url,
            environment_filtering: self.environment_filtering.ok_or_else(|| BuildError::missing_field("environment_filtering"))?,
            explicit_only: self.explicit_only.ok_or_else(|| BuildError::missing_field("explicit_only"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            onboarded_at: self.onboarded_at,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            primary_environment_id: self.primary_environment_id,
            primary_environment_name: self.primary_environment_name,
            sdk_config: self.sdk_config,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            source_datasource_id: self.source_datasource_id,
            source_connection_id: self.source_connection_id,
            source_mapping_id: self.source_mapping_id,
            source_mapping_label: self.source_mapping_label,
            source_mapping_url: self.source_mapping_url,
            source_working_directory: self.source_working_directory,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
