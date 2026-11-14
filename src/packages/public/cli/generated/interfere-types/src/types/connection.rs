pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Connection {
    #[serde(rename = "enabledByUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_by_user_id: Option<String>,
    #[serde(rename = "externalAccountId")]
    #[serde(default)]
    pub external_account_id: String,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    pub provider: ConnectionProvider,
    #[serde(rename = "providerMetadata")]
    pub provider_metadata: ConnectionProviderMetadata,
    pub status: ConnectionStatus,
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<ConnectionStatusReason>,
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

impl Connection {
    pub fn builder() -> ConnectionBuilder {
        <ConnectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionBuilder {
    enabled_by_user_id: Option<String>,
    external_account_id: Option<String>,
    id: Option<String>,
    organization_id: Option<String>,
    provider: Option<ConnectionProvider>,
    provider_metadata: Option<ConnectionProviderMetadata>,
    status: Option<ConnectionStatus>,
    status_reason: Option<ConnectionStatusReason>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
}

impl ConnectionBuilder {
    pub fn enabled_by_user_id(mut self, value: impl Into<String>) -> Self {
        self.enabled_by_user_id = Some(value.into());
        self
    }

    pub fn external_account_id(mut self, value: impl Into<String>) -> Self {
        self.external_account_id = Some(value.into());
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

    pub fn provider(mut self, value: ConnectionProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn provider_metadata(mut self, value: ConnectionProviderMetadata) -> Self {
        self.provider_metadata = Some(value);
        self
    }

    pub fn status(mut self, value: ConnectionStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn status_reason(mut self, value: ConnectionStatusReason) -> Self {
        self.status_reason = Some(value);
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

    /// Consumes the builder and constructs a [`Connection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`external_account_id`](ConnectionBuilder::external_account_id)
    /// - [`id`](ConnectionBuilder::id)
    /// - [`organization_id`](ConnectionBuilder::organization_id)
    /// - [`provider`](ConnectionBuilder::provider)
    /// - [`provider_metadata`](ConnectionBuilder::provider_metadata)
    /// - [`status`](ConnectionBuilder::status)
    /// - [`created_at`](ConnectionBuilder::created_at)
    /// - [`updated_at`](ConnectionBuilder::updated_at)
    pub fn build(self) -> Result<Connection, BuildError> {
        Ok(Connection {
            enabled_by_user_id: self.enabled_by_user_id,
            external_account_id: self.external_account_id.ok_or_else(|| BuildError::missing_field("external_account_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            provider_metadata: self.provider_metadata.ok_or_else(|| BuildError::missing_field("provider_metadata"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            status_reason: self.status_reason,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
