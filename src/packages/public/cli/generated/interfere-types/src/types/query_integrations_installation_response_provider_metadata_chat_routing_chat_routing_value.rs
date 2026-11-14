pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsInstallationResponseProviderMetadataChatRoutingChatRoutingValue {
    #[serde(rename = "channelId")]
    #[serde(default)]
    pub channel_id: String,
    #[serde(rename = "channelName")]
    #[serde(default)]
    pub channel_name: String,
}

impl QueryIntegrationsInstallationResponseProviderMetadataChatRoutingChatRoutingValue {
    pub fn builder() -> QueryIntegrationsInstallationResponseProviderMetadataChatRoutingChatRoutingValueBuilder {
        <QueryIntegrationsInstallationResponseProviderMetadataChatRoutingChatRoutingValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsInstallationResponseProviderMetadataChatRoutingChatRoutingValueBuilder {
    channel_id: Option<String>,
    channel_name: Option<String>,
}

impl QueryIntegrationsInstallationResponseProviderMetadataChatRoutingChatRoutingValueBuilder {
    pub fn channel_id(mut self, value: impl Into<String>) -> Self {
        self.channel_id = Some(value.into());
        self
    }

    pub fn channel_name(mut self, value: impl Into<String>) -> Self {
        self.channel_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsInstallationResponseProviderMetadataChatRoutingChatRoutingValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`channel_id`](QueryIntegrationsInstallationResponseProviderMetadataChatRoutingChatRoutingValueBuilder::channel_id)
    /// - [`channel_name`](QueryIntegrationsInstallationResponseProviderMetadataChatRoutingChatRoutingValueBuilder::channel_name)
    pub fn build(self) -> Result<QueryIntegrationsInstallationResponseProviderMetadataChatRoutingChatRoutingValue, BuildError> {
        Ok(QueryIntegrationsInstallationResponseProviderMetadataChatRoutingChatRoutingValue {
            channel_id: self.channel_id.ok_or_else(|| BuildError::missing_field("channel_id"))?,
            channel_name: self.channel_name.ok_or_else(|| BuildError::missing_field("channel_name"))?,
        })
    }
}
