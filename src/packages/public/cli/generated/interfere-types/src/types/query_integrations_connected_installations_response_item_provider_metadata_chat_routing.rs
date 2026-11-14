pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRouting {
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
    pub chat_routing: Option<HashMap<String, QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingChatRoutingValue>>,
    pub provider: QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingProvider,
    #[serde(rename = "oauthScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_scopes: Option<Vec<String>>,
}

impl QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRouting {
    pub fn builder() -> QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingBuilder {
        <QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingBuilder {
    team_id: Option<String>,
    team_name: Option<String>,
    installation_url: Option<String>,
    chat_routing: Option<HashMap<String, QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingChatRoutingValue>>,
    provider: Option<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingProvider>,
    oauth_scopes: Option<Vec<String>>,
}

impl QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingBuilder {
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

    pub fn chat_routing(mut self, value: HashMap<String, QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingChatRoutingValue>) -> Self {
        self.chat_routing = Some(value);
        self
    }

    pub fn provider(mut self, value: QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn oauth_scopes(mut self, value: Vec<String>) -> Self {
        self.oauth_scopes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRouting`].
    /// This method will fail if any of the following fields are not set:
    /// - [`team_id`](QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingBuilder::team_id)
    /// - [`team_name`](QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingBuilder::team_name)
    /// - [`provider`](QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingBuilder::provider)
    pub fn build(self) -> Result<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRouting, BuildError> {
        Ok(QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRouting {
            team_id: self.team_id.ok_or_else(|| BuildError::missing_field("team_id"))?,
            team_name: self.team_name.ok_or_else(|| BuildError::missing_field("team_name"))?,
            installation_url: self.installation_url,
            chat_routing: self.chat_routing,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            oauth_scopes: self.oauth_scopes,
        })
    }
}
