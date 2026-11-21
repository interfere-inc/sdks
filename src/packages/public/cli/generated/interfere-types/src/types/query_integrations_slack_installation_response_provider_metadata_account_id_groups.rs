pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdGroups {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdGroupsStatus>,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "syncedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synced_at: Option<i64>,
}

impl QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdGroups {
    pub fn builder() -> QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdGroupsBuilder {
        <QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdGroupsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdGroupsBuilder {
    account: Option<String>,
    enabled: Option<bool>,
    status: Option<QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdGroupsStatus>,
    synced_at: Option<i64>,
}

impl QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdGroupsBuilder {
    pub fn account(mut self, value: impl Into<String>) -> Self {
        self.account = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn status(mut self, value: QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdGroupsStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn synced_at(mut self, value: i64) -> Self {
        self.synced_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdGroups`].
    pub fn build(self) -> Result<QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdGroups, BuildError> {
        Ok(QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdGroups {
            account: self.account,
            enabled: self.enabled,
            status: self.status,
            synced_at: self.synced_at,
        })
    }
}
