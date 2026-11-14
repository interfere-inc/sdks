pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserId {
    pub provider: QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdProvider,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module1: Option<QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdModule1>,
    #[serde(rename = "oauthScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_scopes: Option<Vec<String>>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "urlKey")]
    #[serde(default)]
    pub url_key: String,
    #[serde(rename = "appUserId")]
    #[serde(default)]
    pub app_user_id: String,
    #[serde(rename = "installedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_at: Option<i64>,
    #[serde(rename = "permissionsChangedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_changed_at: Option<i64>,
    #[serde(default)]
    pub scopes: Vec<String>,
    #[serde(rename = "installationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

impl QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserId {
    pub fn builder() -> QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdBuilder {
        <QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdBuilder {
    provider: Option<QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdProvider>,
    module1: Option<QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdModule1>,
    oauth_scopes: Option<Vec<String>>,
    id: Option<String>,
    name: Option<String>,
    url_key: Option<String>,
    app_user_id: Option<String>,
    installed_at: Option<i64>,
    permissions_changed_at: Option<i64>,
    scopes: Option<Vec<String>>,
    installation_url: Option<String>,
    avatar: Option<String>,
}

impl QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdBuilder {
    pub fn provider(mut self, value: QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn module1(mut self, value: QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdModule1) -> Self {
        self.module1 = Some(value);
        self
    }

    pub fn oauth_scopes(mut self, value: Vec<String>) -> Self {
        self.oauth_scopes = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn url_key(mut self, value: impl Into<String>) -> Self {
        self.url_key = Some(value.into());
        self
    }

    pub fn app_user_id(mut self, value: impl Into<String>) -> Self {
        self.app_user_id = Some(value.into());
        self
    }

    pub fn installed_at(mut self, value: i64) -> Self {
        self.installed_at = Some(value);
        self
    }

    pub fn permissions_changed_at(mut self, value: i64) -> Self {
        self.permissions_changed_at = Some(value);
        self
    }

    pub fn scopes(mut self, value: Vec<String>) -> Self {
        self.scopes = Some(value);
        self
    }

    pub fn installation_url(mut self, value: impl Into<String>) -> Self {
        self.installation_url = Some(value.into());
        self
    }

    pub fn avatar(mut self, value: impl Into<String>) -> Self {
        self.avatar = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdBuilder::provider)
    /// - [`id`](QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdBuilder::id)
    /// - [`name`](QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdBuilder::name)
    /// - [`url_key`](QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdBuilder::url_key)
    /// - [`app_user_id`](QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdBuilder::app_user_id)
    /// - [`scopes`](QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdBuilder::scopes)
    pub fn build(self) -> Result<QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserId, BuildError> {
        Ok(QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserId {
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            module1: self.module1,
            oauth_scopes: self.oauth_scopes,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            url_key: self.url_key.ok_or_else(|| BuildError::missing_field("url_key"))?,
            app_user_id: self.app_user_id.ok_or_else(|| BuildError::missing_field("app_user_id"))?,
            installed_at: self.installed_at,
            permissions_changed_at: self.permissions_changed_at,
            scopes: self.scopes.ok_or_else(|| BuildError::missing_field("scopes"))?,
            installation_url: self.installation_url,
            avatar: self.avatar,
        })
    }
}
