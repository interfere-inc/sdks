pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryInvitationsForOrgResponseItem {
    #[serde(rename = "clerkInvitationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clerk_invitation_id: Option<String>,
    #[serde(rename = "emailAddress")]
    #[serde(default)]
    pub email_address: String,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "invitedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub invited_at: f64,
    #[serde(rename = "inviterUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inviter_user_id: Option<String>,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(rename = "revokedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub revoked_at: Option<f64>,
    #[serde(default)]
    pub role: String,
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
}

impl QueryInvitationsForOrgResponseItem {
    pub fn builder() -> QueryInvitationsForOrgResponseItemBuilder {
        <QueryInvitationsForOrgResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryInvitationsForOrgResponseItemBuilder {
    clerk_invitation_id: Option<String>,
    email_address: Option<String>,
    id: Option<String>,
    invited_at: Option<f64>,
    inviter_user_id: Option<String>,
    organization_id: Option<String>,
    revoked_at: Option<f64>,
    role: Option<String>,
    status: Option<String>,
    user_id: Option<String>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
}

impl QueryInvitationsForOrgResponseItemBuilder {
    pub fn clerk_invitation_id(mut self, value: impl Into<String>) -> Self {
        self.clerk_invitation_id = Some(value.into());
        self
    }

    pub fn email_address(mut self, value: impl Into<String>) -> Self {
        self.email_address = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn invited_at(mut self, value: f64) -> Self {
        self.invited_at = Some(value);
        self
    }

    pub fn inviter_user_id(mut self, value: impl Into<String>) -> Self {
        self.inviter_user_id = Some(value.into());
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn revoked_at(mut self, value: f64) -> Self {
        self.revoked_at = Some(value);
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
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

    /// Consumes the builder and constructs a [`QueryInvitationsForOrgResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email_address`](QueryInvitationsForOrgResponseItemBuilder::email_address)
    /// - [`id`](QueryInvitationsForOrgResponseItemBuilder::id)
    /// - [`invited_at`](QueryInvitationsForOrgResponseItemBuilder::invited_at)
    /// - [`organization_id`](QueryInvitationsForOrgResponseItemBuilder::organization_id)
    /// - [`role`](QueryInvitationsForOrgResponseItemBuilder::role)
    /// - [`status`](QueryInvitationsForOrgResponseItemBuilder::status)
    /// - [`created_at`](QueryInvitationsForOrgResponseItemBuilder::created_at)
    /// - [`updated_at`](QueryInvitationsForOrgResponseItemBuilder::updated_at)
    pub fn build(self) -> Result<QueryInvitationsForOrgResponseItem, BuildError> {
        Ok(QueryInvitationsForOrgResponseItem {
            clerk_invitation_id: self.clerk_invitation_id,
            email_address: self.email_address.ok_or_else(|| BuildError::missing_field("email_address"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            invited_at: self.invited_at.ok_or_else(|| BuildError::missing_field("invited_at"))?,
            inviter_user_id: self.inviter_user_id,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            revoked_at: self.revoked_at,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            user_id: self.user_id,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
