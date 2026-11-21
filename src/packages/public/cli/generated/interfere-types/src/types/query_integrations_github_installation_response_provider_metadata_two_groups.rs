pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsGithubInstallationResponseProviderMetadataTwoGroups {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<QueryIntegrationsGithubInstallationResponseProviderMetadataTwoGroupsStatus>,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "syncedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synced_at: Option<i64>,
}

impl QueryIntegrationsGithubInstallationResponseProviderMetadataTwoGroups {
    pub fn builder() -> QueryIntegrationsGithubInstallationResponseProviderMetadataTwoGroupsBuilder {
        <QueryIntegrationsGithubInstallationResponseProviderMetadataTwoGroupsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsGithubInstallationResponseProviderMetadataTwoGroupsBuilder {
    account: Option<String>,
    enabled: Option<bool>,
    status: Option<QueryIntegrationsGithubInstallationResponseProviderMetadataTwoGroupsStatus>,
    synced_at: Option<i64>,
}

impl QueryIntegrationsGithubInstallationResponseProviderMetadataTwoGroupsBuilder {
    pub fn account(mut self, value: impl Into<String>) -> Self {
        self.account = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn status(mut self, value: QueryIntegrationsGithubInstallationResponseProviderMetadataTwoGroupsStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn synced_at(mut self, value: i64) -> Self {
        self.synced_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsGithubInstallationResponseProviderMetadataTwoGroups`].
    pub fn build(self) -> Result<QueryIntegrationsGithubInstallationResponseProviderMetadataTwoGroups, BuildError> {
        Ok(QueryIntegrationsGithubInstallationResponseProviderMetadataTwoGroups {
            account: self.account,
            enabled: self.enabled,
            status: self.status,
            synced_at: self.synced_at,
        })
    }
}
