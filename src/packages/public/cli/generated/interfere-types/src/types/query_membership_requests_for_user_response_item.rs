pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryMembershipRequestsForUserResponseItem {
    #[serde(rename = "emailAddress")]
    #[serde(default)]
    pub email_address: String,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(rename = "requestedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub requested_at: f64,
    #[serde(rename = "resolvedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub resolved_at: Option<f64>,
    #[serde(default)]
    pub status: String,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
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
    pub organization: Option<QueryMembershipRequestsForUserResponseItemOrganization>,
}

impl QueryMembershipRequestsForUserResponseItem {
    pub fn builder() -> QueryMembershipRequestsForUserResponseItemBuilder {
        <QueryMembershipRequestsForUserResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryMembershipRequestsForUserResponseItemBuilder {
    email_address: Option<String>,
    id: Option<String>,
    organization_id: Option<String>,
    requested_at: Option<f64>,
    resolved_at: Option<f64>,
    status: Option<String>,
    user_id: Option<String>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
    organization: Option<QueryMembershipRequestsForUserResponseItemOrganization>,
}

impl QueryMembershipRequestsForUserResponseItemBuilder {
    pub fn email_address(mut self, value: impl Into<String>) -> Self {
        self.email_address = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn requested_at(mut self, value: f64) -> Self {
        self.requested_at = Some(value);
        self
    }

    pub fn resolved_at(mut self, value: f64) -> Self {
        self.resolved_at = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
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

    pub fn organization(mut self, value: QueryMembershipRequestsForUserResponseItemOrganization) -> Self {
        self.organization = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryMembershipRequestsForUserResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email_address`](QueryMembershipRequestsForUserResponseItemBuilder::email_address)
    /// - [`id`](QueryMembershipRequestsForUserResponseItemBuilder::id)
    /// - [`organization_id`](QueryMembershipRequestsForUserResponseItemBuilder::organization_id)
    /// - [`requested_at`](QueryMembershipRequestsForUserResponseItemBuilder::requested_at)
    /// - [`status`](QueryMembershipRequestsForUserResponseItemBuilder::status)
    /// - [`created_at`](QueryMembershipRequestsForUserResponseItemBuilder::created_at)
    /// - [`updated_at`](QueryMembershipRequestsForUserResponseItemBuilder::updated_at)
    pub fn build(self) -> Result<QueryMembershipRequestsForUserResponseItem, BuildError> {
        Ok(QueryMembershipRequestsForUserResponseItem {
            email_address: self.email_address.ok_or_else(|| BuildError::missing_field("email_address"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            requested_at: self.requested_at.ok_or_else(|| BuildError::missing_field("requested_at"))?,
            resolved_at: self.resolved_at,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            user_id: self.user_id,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            organization: self.organization,
        })
    }
}
