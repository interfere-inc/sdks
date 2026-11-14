pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryIntegrationsVercelInstallationResponseProviderMetadataChatRouting {
    #[serde(rename = "teamId")]
    #[serde(default)]
    pub team_id: String,
    #[serde(rename = "teamName")]
    #[serde(default)]
    pub team_name: String,
    #[serde(rename = "installationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_url: Option<String>,
    #[serde(rename = "chatRouting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_routing: Option<HashMap<String, QueryIntegrationsVercelInstallationResponseProviderMetadataChatRoutingChatRoutingValue>>,
    pub provider: QueryIntegrationsVercelInstallationResponseProviderMetadataChatRoutingProvider,
    #[serde(rename = "oauthScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_scopes: Option<Vec<String>>,
}

impl QueryIntegrationsVercelInstallationResponseProviderMetadataChatRouting {
    pub fn builder() -> QueryIntegrationsVercelInstallationResponseProviderMetadataChatRoutingBuilder {
        <QueryIntegrationsVercelInstallationResponseProviderMetadataChatRoutingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsVercelInstallationResponseProviderMetadataChatRoutingBuilder {
    team_id: Option<String>,
    team_name: Option<String>,
    installation_url: Option<String>,
    chat_routing: Option<HashMap<String, QueryIntegrationsVercelInstallationResponseProviderMetadataChatRoutingChatRoutingValue>>,
    provider: Option<QueryIntegrationsVercelInstallationResponseProviderMetadataChatRoutingProvider>,
    oauth_scopes: Option<Vec<String>>,
}

impl QueryIntegrationsVercelInstallationResponseProviderMetadataChatRoutingBuilder {
    pub fn team_id(mut self, value: impl Into<String>) -> Self {
        self.team_id = Some(value.into());
        self
    }

    pub fn team_name(mut self, value: impl Into<String>) -> Self {
        self.team_name = Some(value.into());
        self
    }

    pub fn installation_url(mut self, value: impl Into<String>) -> Self {
        self.installation_url = Some(value.into());
        self
    }

    pub fn chat_routing(mut self, value: HashMap<String, QueryIntegrationsVercelInstallationResponseProviderMetadataChatRoutingChatRoutingValue>) -> Self {
        self.chat_routing = Some(value);
        self
    }

    pub fn provider(mut self, value: QueryIntegrationsVercelInstallationResponseProviderMetadataChatRoutingProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn oauth_scopes(mut self, value: Vec<String>) -> Self {
        self.oauth_scopes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsVercelInstallationResponseProviderMetadataChatRouting`].
    /// This method will fail if any of the following fields are not set:
    /// - [`team_id`](QueryIntegrationsVercelInstallationResponseProviderMetadataChatRoutingBuilder::team_id)
    /// - [`team_name`](QueryIntegrationsVercelInstallationResponseProviderMetadataChatRoutingBuilder::team_name)
    /// - [`provider`](QueryIntegrationsVercelInstallationResponseProviderMetadataChatRoutingBuilder::provider)
    pub fn build(self) -> Result<QueryIntegrationsVercelInstallationResponseProviderMetadataChatRouting, BuildError> {
        Ok(QueryIntegrationsVercelInstallationResponseProviderMetadataChatRouting {
            team_id: self.team_id.ok_or_else(|| BuildError::missing_field("team_id"))?,
            team_name: self.team_name.ok_or_else(|| BuildError::missing_field("team_name"))?,
            installation_url: self.installation_url,
            chat_routing: self.chat_routing,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            oauth_scopes: self.oauth_scopes,
        })
    }
}
