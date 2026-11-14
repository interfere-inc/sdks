pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsGithubInstallationResponseProviderMetadataChatRoutingChatRoutingValue {
    #[serde(rename = "channelId")]
    #[serde(default)]
    pub channel_id: String,
    #[serde(rename = "channelName")]
    #[serde(default)]
    pub channel_name: String,
}

impl QueryIntegrationsGithubInstallationResponseProviderMetadataChatRoutingChatRoutingValue {
    pub fn builder() -> QueryIntegrationsGithubInstallationResponseProviderMetadataChatRoutingChatRoutingValueBuilder {
        <QueryIntegrationsGithubInstallationResponseProviderMetadataChatRoutingChatRoutingValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsGithubInstallationResponseProviderMetadataChatRoutingChatRoutingValueBuilder {
    channel_id: Option<String>,
    channel_name: Option<String>,
}

impl QueryIntegrationsGithubInstallationResponseProviderMetadataChatRoutingChatRoutingValueBuilder {
    pub fn channel_id(mut self, value: impl Into<String>) -> Self {
        self.channel_id = Some(value.into());
        self
    }

    pub fn channel_name(mut self, value: impl Into<String>) -> Self {
        self.channel_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsGithubInstallationResponseProviderMetadataChatRoutingChatRoutingValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`channel_id`](QueryIntegrationsGithubInstallationResponseProviderMetadataChatRoutingChatRoutingValueBuilder::channel_id)
    /// - [`channel_name`](QueryIntegrationsGithubInstallationResponseProviderMetadataChatRoutingChatRoutingValueBuilder::channel_name)
    pub fn build(self) -> Result<QueryIntegrationsGithubInstallationResponseProviderMetadataChatRoutingChatRoutingValue, BuildError> {
        Ok(QueryIntegrationsGithubInstallationResponseProviderMetadataChatRoutingChatRoutingValue {
            channel_id: self.channel_id.ok_or_else(|| BuildError::missing_field("channel_id"))?,
            channel_name: self.channel_name.ok_or_else(|| BuildError::missing_field("channel_name"))?,
        })
    }
}
