pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeGroups {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeGroupsStatus>,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "syncedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synced_at: Option<i64>,
}

impl QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeGroups {
    pub fn builder() -> QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeGroupsBuilder {
        <QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeGroupsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeGroupsBuilder {
    account: Option<String>,
    enabled: Option<bool>,
    status: Option<QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeGroupsStatus>,
    synced_at: Option<i64>,
}

impl QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeGroupsBuilder {
    pub fn account(mut self, value: impl Into<String>) -> Self {
        self.account = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn status(mut self, value: QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeGroupsStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn synced_at(mut self, value: i64) -> Self {
        self.synced_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeGroups`].
    pub fn build(self) -> Result<QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeGroups, BuildError> {
        Ok(QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeGroups {
            account: self.account,
            enabled: self.enabled,
            status: self.status,
            synced_at: self.synced_at,
        })
    }
}
