pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryIntegrationsConnectedInstallationsResponseItem {
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
    pub provider: QueryIntegrationsConnectedInstallationsResponseItemProvider,
    #[serde(rename = "providerMetadata")]
    pub provider_metadata: QueryIntegrationsConnectedInstallationsResponseItemProviderMetadata,
    pub status: QueryIntegrationsConnectedInstallationsResponseItemStatus,
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<QueryIntegrationsConnectedInstallationsResponseItemStatusReason>,
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
    #[serde(rename = "enabledBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_by: Option<QueryIntegrationsConnectedInstallationsResponseItemEnabledBy>,
}

impl QueryIntegrationsConnectedInstallationsResponseItem {
    pub fn builder() -> QueryIntegrationsConnectedInstallationsResponseItemBuilder {
        <QueryIntegrationsConnectedInstallationsResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsConnectedInstallationsResponseItemBuilder {
    enabled_by_user_id: Option<String>,
    external_account_id: Option<String>,
    id: Option<String>,
    organization_id: Option<String>,
    provider: Option<QueryIntegrationsConnectedInstallationsResponseItemProvider>,
    provider_metadata: Option<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadata>,
    status: Option<QueryIntegrationsConnectedInstallationsResponseItemStatus>,
    status_reason: Option<QueryIntegrationsConnectedInstallationsResponseItemStatusReason>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
    enabled_by: Option<QueryIntegrationsConnectedInstallationsResponseItemEnabledBy>,
}

impl QueryIntegrationsConnectedInstallationsResponseItemBuilder {
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

    pub fn provider(mut self, value: QueryIntegrationsConnectedInstallationsResponseItemProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn provider_metadata(mut self, value: QueryIntegrationsConnectedInstallationsResponseItemProviderMetadata) -> Self {
        self.provider_metadata = Some(value);
        self
    }

    pub fn status(mut self, value: QueryIntegrationsConnectedInstallationsResponseItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn status_reason(mut self, value: QueryIntegrationsConnectedInstallationsResponseItemStatusReason) -> Self {
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

    pub fn enabled_by(mut self, value: QueryIntegrationsConnectedInstallationsResponseItemEnabledBy) -> Self {
        self.enabled_by = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsConnectedInstallationsResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`external_account_id`](QueryIntegrationsConnectedInstallationsResponseItemBuilder::external_account_id)
    /// - [`id`](QueryIntegrationsConnectedInstallationsResponseItemBuilder::id)
    /// - [`organization_id`](QueryIntegrationsConnectedInstallationsResponseItemBuilder::organization_id)
    /// - [`provider`](QueryIntegrationsConnectedInstallationsResponseItemBuilder::provider)
    /// - [`provider_metadata`](QueryIntegrationsConnectedInstallationsResponseItemBuilder::provider_metadata)
    /// - [`status`](QueryIntegrationsConnectedInstallationsResponseItemBuilder::status)
    /// - [`created_at`](QueryIntegrationsConnectedInstallationsResponseItemBuilder::created_at)
    /// - [`updated_at`](QueryIntegrationsConnectedInstallationsResponseItemBuilder::updated_at)
    pub fn build(self) -> Result<QueryIntegrationsConnectedInstallationsResponseItem, BuildError> {
        Ok(QueryIntegrationsConnectedInstallationsResponseItem {
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
            enabled_by: self.enabled_by,
        })
    }
}
