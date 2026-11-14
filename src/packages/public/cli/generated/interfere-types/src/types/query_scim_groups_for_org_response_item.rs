pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryScimGroupsForOrgResponseItem {
    #[serde(rename = "clerkScimDirectoryId")]
    #[serde(default)]
    pub clerk_scim_directory_id: String,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "memberCount")]
    #[serde(default)]
    pub member_count: i64,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precedence: Option<i64>,
    #[serde(rename = "roleKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_key: Option<String>,
    #[serde(rename = "scimGroupDisplayName")]
    #[serde(default)]
    pub scim_group_display_name: String,
    #[serde(rename = "scimGroupId")]
    #[serde(default)]
    pub scim_group_id: String,
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

impl QueryScimGroupsForOrgResponseItem {
    pub fn builder() -> QueryScimGroupsForOrgResponseItemBuilder {
        <QueryScimGroupsForOrgResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryScimGroupsForOrgResponseItemBuilder {
    clerk_scim_directory_id: Option<String>,
    id: Option<String>,
    member_count: Option<i64>,
    organization_id: Option<String>,
    precedence: Option<i64>,
    role_key: Option<String>,
    scim_group_display_name: Option<String>,
    scim_group_id: Option<String>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
}

impl QueryScimGroupsForOrgResponseItemBuilder {
    pub fn clerk_scim_directory_id(mut self, value: impl Into<String>) -> Self {
        self.clerk_scim_directory_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn member_count(mut self, value: i64) -> Self {
        self.member_count = Some(value);
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn precedence(mut self, value: i64) -> Self {
        self.precedence = Some(value);
        self
    }

    pub fn role_key(mut self, value: impl Into<String>) -> Self {
        self.role_key = Some(value.into());
        self
    }

    pub fn scim_group_display_name(mut self, value: impl Into<String>) -> Self {
        self.scim_group_display_name = Some(value.into());
        self
    }

    pub fn scim_group_id(mut self, value: impl Into<String>) -> Self {
        self.scim_group_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`QueryScimGroupsForOrgResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`clerk_scim_directory_id`](QueryScimGroupsForOrgResponseItemBuilder::clerk_scim_directory_id)
    /// - [`id`](QueryScimGroupsForOrgResponseItemBuilder::id)
    /// - [`member_count`](QueryScimGroupsForOrgResponseItemBuilder::member_count)
    /// - [`organization_id`](QueryScimGroupsForOrgResponseItemBuilder::organization_id)
    /// - [`scim_group_display_name`](QueryScimGroupsForOrgResponseItemBuilder::scim_group_display_name)
    /// - [`scim_group_id`](QueryScimGroupsForOrgResponseItemBuilder::scim_group_id)
    /// - [`created_at`](QueryScimGroupsForOrgResponseItemBuilder::created_at)
    /// - [`updated_at`](QueryScimGroupsForOrgResponseItemBuilder::updated_at)
    pub fn build(self) -> Result<QueryScimGroupsForOrgResponseItem, BuildError> {
        Ok(QueryScimGroupsForOrgResponseItem {
            clerk_scim_directory_id: self.clerk_scim_directory_id.ok_or_else(|| BuildError::missing_field("clerk_scim_directory_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            member_count: self.member_count.ok_or_else(|| BuildError::missing_field("member_count"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            precedence: self.precedence,
            role_key: self.role_key,
            scim_group_display_name: self.scim_group_display_name.ok_or_else(|| BuildError::missing_field("scim_group_display_name"))?,
            scim_group_id: self.scim_group_id.ok_or_else(|| BuildError::missing_field("scim_group_id"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
