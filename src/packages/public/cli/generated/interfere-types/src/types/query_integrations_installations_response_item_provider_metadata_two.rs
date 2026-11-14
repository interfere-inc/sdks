pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryIntegrationsInstallationsResponseItemProviderMetadataTwo {
    #[serde(rename = "accountType")]
    #[serde(default)]
    pub account_type: String,
    #[serde(rename = "avatarUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavor: Option<QueryIntegrationsInstallationsResponseItemProviderMetadataTwoFlavor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(rename = "installationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_url: Option<String>,
    #[serde(default)]
    pub repositories: HashMap<String, String>,
    #[serde(rename = "appRegistration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_registration: Option<QueryIntegrationsInstallationsResponseItemProviderMetadataTwoAppRegistration>,
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<HashMap<String, String>>,
    #[serde(rename = "requiredPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_permissions: Option<HashMap<String, String>>,
    pub provider: QueryIntegrationsInstallationsResponseItemProviderMetadataTwoProvider,
    #[serde(rename = "repositoryDiscoveryRequestedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_discovery_requested_at: Option<i64>,
    #[serde(rename = "repositoryScans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_scans: Option<HashMap<String, QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValue>>,
}

impl QueryIntegrationsInstallationsResponseItemProviderMetadataTwo {
    pub fn builder() -> QueryIntegrationsInstallationsResponseItemProviderMetadataTwoBuilder {
        <QueryIntegrationsInstallationsResponseItemProviderMetadataTwoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsInstallationsResponseItemProviderMetadataTwoBuilder {
    account_type: Option<String>,
    avatar_url: Option<String>,
    flavor: Option<QueryIntegrationsInstallationsResponseItemProviderMetadataTwoFlavor>,
    host: Option<String>,
    installation_url: Option<String>,
    repositories: Option<HashMap<String, String>>,
    app_registration: Option<QueryIntegrationsInstallationsResponseItemProviderMetadataTwoAppRegistration>,
    account_id: Option<String>,
    permissions: Option<HashMap<String, String>>,
    required_permissions: Option<HashMap<String, String>>,
    provider: Option<QueryIntegrationsInstallationsResponseItemProviderMetadataTwoProvider>,
    repository_discovery_requested_at: Option<i64>,
    repository_scans: Option<HashMap<String, QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValue>>,
}

impl QueryIntegrationsInstallationsResponseItemProviderMetadataTwoBuilder {
    pub fn account_type(mut self, value: impl Into<String>) -> Self {
        self.account_type = Some(value.into());
        self
    }

    pub fn avatar_url(mut self, value: impl Into<String>) -> Self {
        self.avatar_url = Some(value.into());
        self
    }

    pub fn flavor(mut self, value: QueryIntegrationsInstallationsResponseItemProviderMetadataTwoFlavor) -> Self {
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

    pub fn app_registration(mut self, value: QueryIntegrationsInstallationsResponseItemProviderMetadataTwoAppRegistration) -> Self {
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

    pub fn provider(mut self, value: QueryIntegrationsInstallationsResponseItemProviderMetadataTwoProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn repository_discovery_requested_at(mut self, value: i64) -> Self {
        self.repository_discovery_requested_at = Some(value);
        self
    }

    pub fn repository_scans(mut self, value: HashMap<String, QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValue>) -> Self {
        self.repository_scans = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsInstallationsResponseItemProviderMetadataTwo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_type`](QueryIntegrationsInstallationsResponseItemProviderMetadataTwoBuilder::account_type)
    /// - [`repositories`](QueryIntegrationsInstallationsResponseItemProviderMetadataTwoBuilder::repositories)
    /// - [`provider`](QueryIntegrationsInstallationsResponseItemProviderMetadataTwoBuilder::provider)
    pub fn build(self) -> Result<QueryIntegrationsInstallationsResponseItemProviderMetadataTwo, BuildError> {
        Ok(QueryIntegrationsInstallationsResponseItemProviderMetadataTwo {
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
