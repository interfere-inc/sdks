pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmail {
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
    pub account_type: QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmailAccountType,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub username: String,
    pub provider: QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmailProvider,
    #[serde(rename = "oauthScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_scopes: Option<Vec<String>>,
    #[serde(rename = "consentCheckedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub consent_checked_at: Option<f64>,
}

impl QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmail {
    pub fn builder() -> QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmailBuilder {
        <QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmailBuilder {
    avatar: Option<String>,
    installation_url: Option<String>,
    name: Option<String>,
    projects: Option<HashMap<String, Option<String>>>,
    account_type: Option<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmailAccountType>,
    email: Option<String>,
    id: Option<String>,
    username: Option<String>,
    provider: Option<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmailProvider>,
    oauth_scopes: Option<Vec<String>>,
    consent_checked_at: Option<f64>,
}

impl QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmailBuilder {
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

    pub fn account_type(mut self, value: QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmailAccountType) -> Self {
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

    pub fn provider(mut self, value: QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmailProvider) -> Self {
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

    /// Consumes the builder and constructs a [`QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_type`](QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmailBuilder::account_type)
    /// - [`email`](QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmailBuilder::email)
    /// - [`id`](QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmailBuilder::id)
    /// - [`username`](QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmailBuilder::username)
    /// - [`provider`](QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmailBuilder::provider)
    pub fn build(self) -> Result<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmail, BuildError> {
        Ok(QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataEmail {
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
