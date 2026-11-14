pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryIntegrationsInstallationResponseProviderMetadataChatRouting {
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
    pub chat_routing: Option<HashMap<String, QueryIntegrationsInstallationResponseProviderMetadataChatRoutingChatRoutingValue>>,
    pub provider: QueryIntegrationsInstallationResponseProviderMetadataChatRoutingProvider,
    #[serde(rename = "oauthScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_scopes: Option<Vec<String>>,
}

impl QueryIntegrationsInstallationResponseProviderMetadataChatRouting {
    pub fn builder() -> QueryIntegrationsInstallationResponseProviderMetadataChatRoutingBuilder {
        <QueryIntegrationsInstallationResponseProviderMetadataChatRoutingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsInstallationResponseProviderMetadataChatRoutingBuilder {
    team_id: Option<String>,
    team_name: Option<String>,
    installation_url: Option<String>,
    chat_routing: Option<HashMap<String, QueryIntegrationsInstallationResponseProviderMetadataChatRoutingChatRoutingValue>>,
    provider: Option<QueryIntegrationsInstallationResponseProviderMetadataChatRoutingProvider>,
    oauth_scopes: Option<Vec<String>>,
}

impl QueryIntegrationsInstallationResponseProviderMetadataChatRoutingBuilder {
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

    pub fn chat_routing(mut self, value: HashMap<String, QueryIntegrationsInstallationResponseProviderMetadataChatRoutingChatRoutingValue>) -> Self {
        self.chat_routing = Some(value);
        self
    }

    pub fn provider(mut self, value: QueryIntegrationsInstallationResponseProviderMetadataChatRoutingProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn oauth_scopes(mut self, value: Vec<String>) -> Self {
        self.oauth_scopes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsInstallationResponseProviderMetadataChatRouting`].
    /// This method will fail if any of the following fields are not set:
    /// - [`team_id`](QueryIntegrationsInstallationResponseProviderMetadataChatRoutingBuilder::team_id)
    /// - [`team_name`](QueryIntegrationsInstallationResponseProviderMetadataChatRoutingBuilder::team_name)
    /// - [`provider`](QueryIntegrationsInstallationResponseProviderMetadataChatRoutingBuilder::provider)
    pub fn build(self) -> Result<QueryIntegrationsInstallationResponseProviderMetadataChatRouting, BuildError> {
        Ok(QueryIntegrationsInstallationResponseProviderMetadataChatRouting {
            team_id: self.team_id.ok_or_else(|| BuildError::missing_field("team_id"))?,
            team_name: self.team_name.ok_or_else(|| BuildError::missing_field("team_name"))?,
            installation_url: self.installation_url,
            chat_routing: self.chat_routing,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            oauth_scopes: self.oauth_scopes,
        })
    }
}
