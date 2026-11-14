pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryIntegrationsGithubInstallationResponse {
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
    pub provider: QueryIntegrationsGithubInstallationResponseProvider,
    #[serde(rename = "providerMetadata")]
    pub provider_metadata: QueryIntegrationsGithubInstallationResponseProviderMetadata,
    pub status: QueryIntegrationsGithubInstallationResponseStatus,
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<QueryIntegrationsGithubInstallationResponseStatusReason>,
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

impl QueryIntegrationsGithubInstallationResponse {
    pub fn builder() -> QueryIntegrationsGithubInstallationResponseBuilder {
        <QueryIntegrationsGithubInstallationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsGithubInstallationResponseBuilder {
    enabled_by_user_id: Option<String>,
    external_account_id: Option<String>,
    id: Option<String>,
    organization_id: Option<String>,
    provider: Option<QueryIntegrationsGithubInstallationResponseProvider>,
    provider_metadata: Option<QueryIntegrationsGithubInstallationResponseProviderMetadata>,
    status: Option<QueryIntegrationsGithubInstallationResponseStatus>,
    status_reason: Option<QueryIntegrationsGithubInstallationResponseStatusReason>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
}

impl QueryIntegrationsGithubInstallationResponseBuilder {
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

    pub fn provider(mut self, value: QueryIntegrationsGithubInstallationResponseProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn provider_metadata(mut self, value: QueryIntegrationsGithubInstallationResponseProviderMetadata) -> Self {
        self.provider_metadata = Some(value);
        self
    }

    pub fn status(mut self, value: QueryIntegrationsGithubInstallationResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn status_reason(mut self, value: QueryIntegrationsGithubInstallationResponseStatusReason) -> Self {
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

    /// Consumes the builder and constructs a [`QueryIntegrationsGithubInstallationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`external_account_id`](QueryIntegrationsGithubInstallationResponseBuilder::external_account_id)
    /// - [`id`](QueryIntegrationsGithubInstallationResponseBuilder::id)
    /// - [`organization_id`](QueryIntegrationsGithubInstallationResponseBuilder::organization_id)
    /// - [`provider`](QueryIntegrationsGithubInstallationResponseBuilder::provider)
    /// - [`provider_metadata`](QueryIntegrationsGithubInstallationResponseBuilder::provider_metadata)
    /// - [`status`](QueryIntegrationsGithubInstallationResponseBuilder::status)
    /// - [`created_at`](QueryIntegrationsGithubInstallationResponseBuilder::created_at)
    /// - [`updated_at`](QueryIntegrationsGithubInstallationResponseBuilder::updated_at)
    pub fn build(self) -> Result<QueryIntegrationsGithubInstallationResponse, BuildError> {
        Ok(QueryIntegrationsGithubInstallationResponse {
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
