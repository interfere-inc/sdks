pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRouting {
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
    pub chat_routing: Option<HashMap<String, QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRoutingChatRoutingValue>>,
    pub provider: QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRoutingProvider,
    #[serde(rename = "oauthScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_scopes: Option<Vec<String>>,
}

impl QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRouting {
    pub fn builder() -> QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRoutingBuilder {
        <QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRoutingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRoutingBuilder {
    team_id: Option<String>,
    team_name: Option<String>,
    installation_url: Option<String>,
    chat_routing: Option<HashMap<String, QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRoutingChatRoutingValue>>,
    provider: Option<QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRoutingProvider>,
    oauth_scopes: Option<Vec<String>>,
}

impl QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRoutingBuilder {
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

    pub fn chat_routing(mut self, value: HashMap<String, QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRoutingChatRoutingValue>) -> Self {
        self.chat_routing = Some(value);
        self
    }

    pub fn provider(mut self, value: QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRoutingProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn oauth_scopes(mut self, value: Vec<String>) -> Self {
        self.oauth_scopes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRouting`].
    /// This method will fail if any of the following fields are not set:
    /// - [`team_id`](QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRoutingBuilder::team_id)
    /// - [`team_name`](QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRoutingBuilder::team_name)
    /// - [`provider`](QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRoutingBuilder::provider)
    pub fn build(self) -> Result<QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRouting, BuildError> {
        Ok(QueryIntegrationsGheCloudInstallationResponseProviderMetadataChatRouting {
            team_id: self.team_id.ok_or_else(|| BuildError::missing_field("team_id"))?,
            team_name: self.team_name.ok_or_else(|| BuildError::missing_field("team_name"))?,
            installation_url: self.installation_url,
            chat_routing: self.chat_routing,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            oauth_scopes: self.oauth_scopes,
        })
    }
}
