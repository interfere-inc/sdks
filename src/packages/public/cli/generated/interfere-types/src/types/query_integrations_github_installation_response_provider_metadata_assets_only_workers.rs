pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkers {
    #[serde(rename = "accountType")]
    pub account_type: QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkersAccountType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "installationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "deploymentCutoffs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_cutoffs: Option<HashMap<String, String>>,
    #[serde(rename = "versionCutoffs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_cutoffs: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workers: Option<HashMap<String, String>>,
    #[serde(rename = "assetsOnlyWorkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets_only_workers: Option<Vec<String>>,
    #[serde(rename = "workersDiscoveredAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workers_discovered_at: Option<i64>,
    pub provider: QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkersProvider,
    #[serde(rename = "oauthScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_scopes: Option<Vec<String>>,
    #[serde(rename = "refreshToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<serde_json::Value>,
}

impl QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkers {
    pub fn builder() -> QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkersBuilder {
        <QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkersBuilder {
    account_type: Option<QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkersAccountType>,
    avatar: Option<String>,
    id: Option<String>,
    installation_url: Option<String>,
    name: Option<String>,
    deployment_cutoffs: Option<HashMap<String, String>>,
    version_cutoffs: Option<HashMap<String, String>>,
    workers: Option<HashMap<String, String>>,
    assets_only_workers: Option<Vec<String>>,
    workers_discovered_at: Option<i64>,
    provider: Option<QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkersProvider>,
    oauth_scopes: Option<Vec<String>>,
    refresh_token: Option<serde_json::Value>,
}

impl QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkersBuilder {
    pub fn account_type(mut self, value: QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkersAccountType) -> Self {
        self.account_type = Some(value);
        self
    }

    pub fn avatar(mut self, value: impl Into<String>) -> Self {
        self.avatar = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn installation_url(mut self, value: impl Into<String>) -> Self {
        self.installation_url = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn deployment_cutoffs(mut self, value: HashMap<String, String>) -> Self {
        self.deployment_cutoffs = Some(value);
        self
    }

    pub fn version_cutoffs(mut self, value: HashMap<String, String>) -> Self {
        self.version_cutoffs = Some(value);
        self
    }

    pub fn workers(mut self, value: HashMap<String, String>) -> Self {
        self.workers = Some(value);
        self
    }

    pub fn assets_only_workers(mut self, value: Vec<String>) -> Self {
        self.assets_only_workers = Some(value);
        self
    }

    pub fn workers_discovered_at(mut self, value: i64) -> Self {
        self.workers_discovered_at = Some(value);
        self
    }

    pub fn provider(mut self, value: QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkersProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn oauth_scopes(mut self, value: Vec<String>) -> Self {
        self.oauth_scopes = Some(value);
        self
    }

    pub fn refresh_token(mut self, value: serde_json::Value) -> Self {
        self.refresh_token = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkers`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_type`](QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkersBuilder::account_type)
    /// - [`id`](QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkersBuilder::id)
    /// - [`provider`](QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkersBuilder::provider)
    pub fn build(self) -> Result<QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkers, BuildError> {
        Ok(QueryIntegrationsGithubInstallationResponseProviderMetadataAssetsOnlyWorkers {
            account_type: self.account_type.ok_or_else(|| BuildError::missing_field("account_type"))?,
            avatar: self.avatar,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            installation_url: self.installation_url,
            name: self.name,
            deployment_cutoffs: self.deployment_cutoffs,
            version_cutoffs: self.version_cutoffs,
            workers: self.workers,
            assets_only_workers: self.assets_only_workers,
            workers_discovered_at: self.workers_discovered_at,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            oauth_scopes: self.oauth_scopes,
            refresh_token: self.refresh_token,
        })
    }
}
