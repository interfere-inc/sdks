pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryIntegrationsGheServerInstallationResponseProviderMetadataSlug {
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
    pub account_type: QueryIntegrationsGheServerInstallationResponseProviderMetadataSlugAccountType,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub slug: String,
    pub provider: QueryIntegrationsGheServerInstallationResponseProviderMetadataSlugProvider,
    #[serde(rename = "oauthScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_scopes: Option<Vec<String>>,
    #[serde(rename = "consentCheckedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub consent_checked_at: Option<f64>,
}

impl QueryIntegrationsGheServerInstallationResponseProviderMetadataSlug {
    pub fn builder() -> QueryIntegrationsGheServerInstallationResponseProviderMetadataSlugBuilder {
        <QueryIntegrationsGheServerInstallationResponseProviderMetadataSlugBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsGheServerInstallationResponseProviderMetadataSlugBuilder {
    avatar: Option<String>,
    installation_url: Option<String>,
    name: Option<String>,
    projects: Option<HashMap<String, Option<String>>>,
    account_type: Option<QueryIntegrationsGheServerInstallationResponseProviderMetadataSlugAccountType>,
    id: Option<String>,
    slug: Option<String>,
    provider: Option<QueryIntegrationsGheServerInstallationResponseProviderMetadataSlugProvider>,
    oauth_scopes: Option<Vec<String>>,
    consent_checked_at: Option<f64>,
}

impl QueryIntegrationsGheServerInstallationResponseProviderMetadataSlugBuilder {
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

    pub fn account_type(mut self, value: QueryIntegrationsGheServerInstallationResponseProviderMetadataSlugAccountType) -> Self {
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

    pub fn provider(mut self, value: QueryIntegrationsGheServerInstallationResponseProviderMetadataSlugProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn oauth_scopes(mut self, value: Vec<String>) -> Self {
        self.oauth_scopes = Some(value);
        self
    }

    pub fn consent_checked_at(mut self, value: f64) -> Self {
        self.consent_checked_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsGheServerInstallationResponseProviderMetadataSlug`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_type`](QueryIntegrationsGheServerInstallationResponseProviderMetadataSlugBuilder::account_type)
    /// - [`id`](QueryIntegrationsGheServerInstallationResponseProviderMetadataSlugBuilder::id)
    /// - [`slug`](QueryIntegrationsGheServerInstallationResponseProviderMetadataSlugBuilder::slug)
    /// - [`provider`](QueryIntegrationsGheServerInstallationResponseProviderMetadataSlugBuilder::provider)
    pub fn build(self) -> Result<QueryIntegrationsGheServerInstallationResponseProviderMetadataSlug, BuildError> {
        Ok(QueryIntegrationsGheServerInstallationResponseProviderMetadataSlug {
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
