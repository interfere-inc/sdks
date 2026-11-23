pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmail {
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
    pub account_type: QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmailAccountType,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub username: String,
    pub provider: QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmailProvider,
    #[serde(rename = "oauthScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_scopes: Option<Vec<String>>,
    #[serde(rename = "consentCheckedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub consent_checked_at: Option<f64>,
}

impl QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmail {
    pub fn builder() -> QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmailBuilder {
        <QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmailBuilder {
    avatar: Option<String>,
    installation_url: Option<String>,
    name: Option<String>,
    projects: Option<HashMap<String, Option<String>>>,
    account_type: Option<QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmailAccountType>,
    email: Option<String>,
    id: Option<String>,
    username: Option<String>,
    provider: Option<QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmailProvider>,
    oauth_scopes: Option<Vec<String>>,
    consent_checked_at: Option<f64>,
}

impl QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmailBuilder {
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

    pub fn account_type(mut self, value: QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmailAccountType) -> Self {
        self.account_type = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    pub fn provider(mut self, value: QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmailProvider) -> Self {
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

    /// Consumes the builder and constructs a [`QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_type`](QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmailBuilder::account_type)
    /// - [`email`](QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmailBuilder::email)
    /// - [`id`](QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmailBuilder::id)
    /// - [`username`](QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmailBuilder::username)
    /// - [`provider`](QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmailBuilder::provider)
    pub fn build(self) -> Result<QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmail, BuildError> {
        Ok(QueryIntegrationsGheCloudInstallationResponseProviderMetadataEmail {
            avatar: self.avatar,
            installation_url: self.installation_url,
            name: self.name,
            projects: self.projects,
            account_type: self.account_type.ok_or_else(|| BuildError::missing_field("account_type"))?,
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            username: self.username.ok_or_else(|| BuildError::missing_field("username"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            oauth_scopes: self.oauth_scopes,
            consent_checked_at: self.consent_checked_at,
        })
    }
}
