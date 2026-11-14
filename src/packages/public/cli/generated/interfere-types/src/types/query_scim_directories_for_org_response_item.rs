pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryScimDirectoriesForOrgResponseItem {
    #[serde(rename = "clerkEnterpriseConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clerk_enterprise_connection_id: Option<String>,
    #[serde(rename = "clerkScimDirectoryId")]
    #[serde(default)]
    pub clerk_scim_directory_id: String,
    #[serde(rename = "clerkUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub clerk_updated_at: Option<f64>,
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "groupRoleMappingEnabled")]
    #[serde(default)]
    pub group_role_mapping_enabled: bool,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(default)]
    pub provider: String,
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

impl QueryScimDirectoriesForOrgResponseItem {
    pub fn builder() -> QueryScimDirectoriesForOrgResponseItemBuilder {
        <QueryScimDirectoriesForOrgResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryScimDirectoriesForOrgResponseItemBuilder {
    clerk_enterprise_connection_id: Option<String>,
    clerk_scim_directory_id: Option<String>,
    clerk_updated_at: Option<f64>,
    enabled: Option<bool>,
    group_role_mapping_enabled: Option<bool>,
    id: Option<String>,
    name: Option<String>,
    organization_id: Option<String>,
    provider: Option<String>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
}

impl QueryScimDirectoriesForOrgResponseItemBuilder {
    pub fn clerk_enterprise_connection_id(mut self, value: impl Into<String>) -> Self {
        self.clerk_enterprise_connection_id = Some(value.into());
        self
    }

    pub fn clerk_scim_directory_id(mut self, value: impl Into<String>) -> Self {
        self.clerk_scim_directory_id = Some(value.into());
        self
    }

    pub fn clerk_updated_at(mut self, value: f64) -> Self {
        self.clerk_updated_at = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn group_role_mapping_enabled(mut self, value: bool) -> Self {
        self.group_role_mapping_enabled = Some(value);
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

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
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

    /// Consumes the builder and constructs a [`QueryScimDirectoriesForOrgResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`clerk_scim_directory_id`](QueryScimDirectoriesForOrgResponseItemBuilder::clerk_scim_directory_id)
    /// - [`enabled`](QueryScimDirectoriesForOrgResponseItemBuilder::enabled)
    /// - [`group_role_mapping_enabled`](QueryScimDirectoriesForOrgResponseItemBuilder::group_role_mapping_enabled)
    /// - [`id`](QueryScimDirectoriesForOrgResponseItemBuilder::id)
    /// - [`name`](QueryScimDirectoriesForOrgResponseItemBuilder::name)
    /// - [`organization_id`](QueryScimDirectoriesForOrgResponseItemBuilder::organization_id)
    /// - [`provider`](QueryScimDirectoriesForOrgResponseItemBuilder::provider)
    /// - [`created_at`](QueryScimDirectoriesForOrgResponseItemBuilder::created_at)
    /// - [`updated_at`](QueryScimDirectoriesForOrgResponseItemBuilder::updated_at)
    pub fn build(self) -> Result<QueryScimDirectoriesForOrgResponseItem, BuildError> {
        Ok(QueryScimDirectoriesForOrgResponseItem {
            clerk_enterprise_connection_id: self.clerk_enterprise_connection_id,
            clerk_scim_directory_id: self.clerk_scim_directory_id.ok_or_else(|| BuildError::missing_field("clerk_scim_directory_id"))?,
            clerk_updated_at: self.clerk_updated_at,
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
            group_role_mapping_enabled: self.group_role_mapping_enabled.ok_or_else(|| BuildError::missing_field("group_role_mapping_enabled"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
