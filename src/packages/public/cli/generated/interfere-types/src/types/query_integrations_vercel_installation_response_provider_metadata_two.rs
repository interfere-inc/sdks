pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryIntegrationsVercelInstallationResponseProviderMetadataTwo {
    #[serde(rename = "accountType")]
    #[serde(default)]
    pub account_type: String,
    #[serde(rename = "avatarUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavor: Option<QueryIntegrationsVercelInstallationResponseProviderMetadataTwoFlavor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(rename = "installationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_url: Option<String>,
    #[serde(default)]
    pub repositories: HashMap<String, String>,
    #[serde(rename = "appRegistration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_registration: Option<QueryIntegrationsVercelInstallationResponseProviderMetadataTwoAppRegistration>,
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<HashMap<String, String>>,
    #[serde(rename = "requiredPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_permissions: Option<HashMap<String, String>>,
    pub provider: QueryIntegrationsVercelInstallationResponseProviderMetadataTwoProvider,
    #[serde(rename = "repositoryDiscoveryRequestedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_discovery_requested_at: Option<i64>,
    #[serde(rename = "repositoryScans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_scans: Option<HashMap<String, QueryIntegrationsVercelInstallationResponseProviderMetadataTwoRepositoryScansValue>>,
}

impl QueryIntegrationsVercelInstallationResponseProviderMetadataTwo {
    pub fn builder() -> QueryIntegrationsVercelInstallationResponseProviderMetadataTwoBuilder {
        <QueryIntegrationsVercelInstallationResponseProviderMetadataTwoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsVercelInstallationResponseProviderMetadataTwoBuilder {
    account_type: Option<String>,
    avatar_url: Option<String>,
    flavor: Option<QueryIntegrationsVercelInstallationResponseProviderMetadataTwoFlavor>,
    host: Option<String>,
    installation_url: Option<String>,
    repositories: Option<HashMap<String, String>>,
    app_registration: Option<QueryIntegrationsVercelInstallationResponseProviderMetadataTwoAppRegistration>,
    account_id: Option<String>,
    permissions: Option<HashMap<String, String>>,
    required_permissions: Option<HashMap<String, String>>,
    provider: Option<QueryIntegrationsVercelInstallationResponseProviderMetadataTwoProvider>,
    repository_discovery_requested_at: Option<i64>,
    repository_scans: Option<HashMap<String, QueryIntegrationsVercelInstallationResponseProviderMetadataTwoRepositoryScansValue>>,
}

impl QueryIntegrationsVercelInstallationResponseProviderMetadataTwoBuilder {
    pub fn account_type(mut self, value: impl Into<String>) -> Self {
        self.account_type = Some(value.into());
        self
    }

    pub fn avatar_url(mut self, value: impl Into<String>) -> Self {
        self.avatar_url = Some(value.into());
        self
    }

    pub fn flavor(mut self, value: QueryIntegrationsVercelInstallationResponseProviderMetadataTwoFlavor) -> Self {
        self.flavor = Some(value);
        self
    }

    pub fn host(mut self, value: impl Into<String>) -> Self {
        self.host = Some(value.into());
        self
    }

    pub fn installation_url(mut self, value: impl Into<String>) -> Self {
        self.installation_url = Some(value.into());
        self
    }

    pub fn repositories(mut self, value: HashMap<String, String>) -> Self {
        self.repositories = Some(value);
        self
    }

    pub fn app_registration(mut self, value: QueryIntegrationsVercelInstallationResponseProviderMetadataTwoAppRegistration) -> Self {
        self.app_registration = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn permissions(mut self, value: HashMap<String, String>) -> Self {
        self.permissions = Some(value);
        self
    }

    pub fn required_permissions(mut self, value: HashMap<String, String>) -> Self {
        self.required_permissions = Some(value);
        self
    }

    pub fn provider(mut self, value: QueryIntegrationsVercelInstallationResponseProviderMetadataTwoProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn repository_discovery_requested_at(mut self, value: i64) -> Self {
        self.repository_discovery_requested_at = Some(value);
        self
    }

    pub fn repository_scans(mut self, value: HashMap<String, QueryIntegrationsVercelInstallationResponseProviderMetadataTwoRepositoryScansValue>) -> Self {
        self.repository_scans = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsVercelInstallationResponseProviderMetadataTwo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_type`](QueryIntegrationsVercelInstallationResponseProviderMetadataTwoBuilder::account_type)
    /// - [`repositories`](QueryIntegrationsVercelInstallationResponseProviderMetadataTwoBuilder::repositories)
    /// - [`provider`](QueryIntegrationsVercelInstallationResponseProviderMetadataTwoBuilder::provider)
    pub fn build(self) -> Result<QueryIntegrationsVercelInstallationResponseProviderMetadataTwo, BuildError> {
        Ok(QueryIntegrationsVercelInstallationResponseProviderMetadataTwo {
            account_type: self.account_type.ok_or_else(|| BuildError::missing_field("account_type"))?,
            avatar_url: self.avatar_url,
            flavor: self.flavor,
            host: self.host,
            installation_url: self.installation_url,
            repositories: self.repositories.ok_or_else(|| BuildError::missing_field("repositories"))?,
            app_registration: self.app_registration,
            account_id: self.account_id,
            permissions: self.permissions,
            required_permissions: self.required_permissions,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            repository_discovery_requested_at: self.repository_discovery_requested_at,
            repository_scans: self.repository_scans,
        })
    }
}
