pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryMembershipsForOrgResponseItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "joinedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub joined_at: Option<f64>,
    #[serde(rename = "leftAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub left_at: Option<f64>,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(default)]
    pub role: String,
    #[serde(rename = "userId")]
    #[serde(default)]
    pub user_id: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<QueryMembershipsForOrgResponseItemUser>,
}

impl QueryMembershipsForOrgResponseItem {
    pub fn builder() -> QueryMembershipsForOrgResponseItemBuilder {
        <QueryMembershipsForOrgResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryMembershipsForOrgResponseItemBuilder {
    id: Option<String>,
    joined_at: Option<f64>,
    left_at: Option<f64>,
    organization_id: Option<String>,
    role: Option<String>,
    user_id: Option<String>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
    user: Option<QueryMembershipsForOrgResponseItemUser>,
}

impl QueryMembershipsForOrgResponseItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn joined_at(mut self, value: f64) -> Self {
        self.joined_at = Some(value);
        self
    }

    pub fn left_at(mut self, value: f64) -> Self {
        self.left_at = Some(value);
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
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

    pub fn user(mut self, value: QueryMembershipsForOrgResponseItemUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryMembershipsForOrgResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QueryMembershipsForOrgResponseItemBuilder::id)
    /// - [`organization_id`](QueryMembershipsForOrgResponseItemBuilder::organization_id)
    /// - [`role`](QueryMembershipsForOrgResponseItemBuilder::role)
    /// - [`user_id`](QueryMembershipsForOrgResponseItemBuilder::user_id)
    /// - [`created_at`](QueryMembershipsForOrgResponseItemBuilder::created_at)
    /// - [`updated_at`](QueryMembershipsForOrgResponseItemBuilder::updated_at)
    pub fn build(self) -> Result<QueryMembershipsForOrgResponseItem, BuildError> {
        Ok(QueryMembershipsForOrgResponseItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            joined_at: self.joined_at,
            left_at: self.left_at,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            user_id: self.user_id.ok_or_else(|| BuildError::missing_field("user_id"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            user: self.user,
        })
    }
}
