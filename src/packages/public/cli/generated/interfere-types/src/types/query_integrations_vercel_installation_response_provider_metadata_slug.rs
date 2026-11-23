pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryIntegrationsVercelInstallationResponseProviderMetadataSlug {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(rename = "installationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<HashMap<String, Option<String>>>,
    #[serde(rename = "accountType")]
    pub account_type: QueryIntegrationsVercelInstallationResponseProviderMetadataSlugAccountType,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub slug: String,
    pub provider: QueryIntegrationsVercelInstallationResponseProviderMetadataSlugProvider,
    #[serde(rename = "oauthScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_scopes: Option<Vec<String>>,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "consentCheckedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_checked_at: Option<i64>,
}

impl QueryIntegrationsVercelInstallationResponseProviderMetadataSlug {
    pub fn builder() -> QueryIntegrationsVercelInstallationResponseProviderMetadataSlugBuilder {
        <QueryIntegrationsVercelInstallationResponseProviderMetadataSlugBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsVercelInstallationResponseProviderMetadataSlugBuilder {
    avatar: Option<String>,
    installation_url: Option<String>,
    name: Option<String>,
    projects: Option<HashMap<String, Option<String>>>,
    account_type: Option<QueryIntegrationsVercelInstallationResponseProviderMetadataSlugAccountType>,
    id: Option<String>,
    slug: Option<String>,
    provider: Option<QueryIntegrationsVercelInstallationResponseProviderMetadataSlugProvider>,
    oauth_scopes: Option<Vec<String>>,
    consent_checked_at: Option<i64>,
}

impl QueryIntegrationsVercelInstallationResponseProviderMetadataSlugBuilder {
    pub fn avatar(mut self, value: impl Into<String>) -> Self {
        self.avatar = Some(value.into());
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

    pub fn projects(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.projects = Some(value);
        self
    }

    pub fn account_type(mut self, value: QueryIntegrationsVercelInstallationResponseProviderMetadataSlugAccountType) -> Self {
        self.account_type = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn provider(mut self, value: QueryIntegrationsVercelInstallationResponseProviderMetadataSlugProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn oauth_scopes(mut self, value: Vec<String>) -> Self {
        self.oauth_scopes = Some(value);
        self
    }

    pub fn consent_checked_at(mut self, value: i64) -> Self {
        self.consent_checked_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsVercelInstallationResponseProviderMetadataSlug`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_type`](QueryIntegrationsVercelInstallationResponseProviderMetadataSlugBuilder::account_type)
    /// - [`id`](QueryIntegrationsVercelInstallationResponseProviderMetadataSlugBuilder::id)
    /// - [`slug`](QueryIntegrationsVercelInstallationResponseProviderMetadataSlugBuilder::slug)
    /// - [`provider`](QueryIntegrationsVercelInstallationResponseProviderMetadataSlugBuilder::provider)
    pub fn build(self) -> Result<QueryIntegrationsVercelInstallationResponseProviderMetadataSlug, BuildError> {
        Ok(QueryIntegrationsVercelInstallationResponseProviderMetadataSlug {
            avatar: self.avatar,
            installation_url: self.installation_url,
            name: self.name,
            projects: self.projects,
            account_type: self.account_type.ok_or_else(|| BuildError::missing_field("account_type"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            oauth_scopes: self.oauth_scopes,
            consent_checked_at: self.consent_checked_at,
        })
    }
}
