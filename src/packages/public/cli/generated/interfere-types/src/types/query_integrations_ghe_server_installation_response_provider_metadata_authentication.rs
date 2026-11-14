pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthentication {
    pub provider: QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthenticationProvider,
    #[serde(default)]
    pub endpoint: String,
    pub authentication: QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthenticationAuthentication,
    #[serde(rename = "oauthScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_scopes: Option<Vec<String>>,
    #[serde(rename = "requiredOAuthScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_o_auth_scopes: Option<Vec<String>>,
    #[serde(rename = "readToolNames")]
    #[serde(default)]
    pub read_tool_names: Vec<String>,
    #[serde(rename = "resourceUris")]
    #[serde(default)]
    pub resource_uris: Vec<String>,
    #[serde(rename = "connectionRevision")]
    #[serde(default)]
    pub connection_revision: i64,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "installationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_url: Option<String>,
}

impl QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthentication {
    pub fn builder() -> QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthenticationBuilder {
        <QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthenticationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthenticationBuilder {
    provider: Option<QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthenticationProvider>,
    endpoint: Option<String>,
    authentication: Option<QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthenticationAuthentication>,
    oauth_scopes: Option<Vec<String>>,
    required_o_auth_scopes: Option<Vec<String>>,
    read_tool_names: Option<Vec<String>>,
    resource_uris: Option<Vec<String>>,
    connection_revision: Option<i64>,
    name: Option<String>,
    installation_url: Option<String>,
}

impl QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthenticationBuilder {
    pub fn provider(mut self, value: QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthenticationProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn endpoint(mut self, value: impl Into<String>) -> Self {
        self.endpoint = Some(value.into());
        self
    }

    pub fn authentication(mut self, value: QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthenticationAuthentication) -> Self {
        self.authentication = Some(value);
        self
    }

    pub fn oauth_scopes(mut self, value: Vec<String>) -> Self {
        self.oauth_scopes = Some(value);
        self
    }

    pub fn required_o_auth_scopes(mut self, value: Vec<String>) -> Self {
        self.required_o_auth_scopes = Some(value);
        self
    }

    pub fn read_tool_names(mut self, value: Vec<String>) -> Self {
        self.read_tool_names = Some(value);
        self
    }

    pub fn resource_uris(mut self, value: Vec<String>) -> Self {
        self.resource_uris = Some(value);
        self
    }

    pub fn connection_revision(mut self, value: i64) -> Self {
        self.connection_revision = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn installation_url(mut self, value: impl Into<String>) -> Self {
        self.installation_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthentication`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthenticationBuilder::provider)
    /// - [`endpoint`](QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthenticationBuilder::endpoint)
    /// - [`authentication`](QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthenticationBuilder::authentication)
    /// - [`read_tool_names`](QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthenticationBuilder::read_tool_names)
    /// - [`resource_uris`](QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthenticationBuilder::resource_uris)
    /// - [`connection_revision`](QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthenticationBuilder::connection_revision)
    /// - [`name`](QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthenticationBuilder::name)
    pub fn build(self) -> Result<QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthentication, BuildError> {
        Ok(QueryIntegrationsGheServerInstallationResponseProviderMetadataAuthentication {
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            endpoint: self.endpoint.ok_or_else(|| BuildError::missing_field("endpoint"))?,
            authentication: self.authentication.ok_or_else(|| BuildError::missing_field("authentication"))?,
            oauth_scopes: self.oauth_scopes,
            required_o_auth_scopes: self.required_o_auth_scopes,
            read_tool_names: self.read_tool_names.ok_or_else(|| BuildError::missing_field("read_tool_names"))?,
            resource_uris: self.resource_uris.ok_or_else(|| BuildError::missing_field("resource_uris"))?,
            connection_revision: self.connection_revision.ok_or_else(|| BuildError::missing_field("connection_revision"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            installation_url: self.installation_url,
        })
    }
}
