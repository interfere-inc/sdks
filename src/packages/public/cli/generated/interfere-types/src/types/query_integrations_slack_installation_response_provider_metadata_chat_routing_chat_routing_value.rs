pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsSlackInstallationResponseProviderMetadataChatRoutingChatRoutingValue {
    #[serde(rename = "channelId")]
    #[serde(default)]
    pub channel_id: String,
    #[serde(rename = "channelName")]
    #[serde(default)]
    pub channel_name: String,
}

impl QueryIntegrationsSlackInstallationResponseProviderMetadataChatRoutingChatRoutingValue {
    pub fn builder() -> QueryIntegrationsSlackInstallationResponseProviderMetadataChatRoutingChatRoutingValueBuilder {
        <QueryIntegrationsSlackInstallationResponseProviderMetadataChatRoutingChatRoutingValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsSlackInstallationResponseProviderMetadataChatRoutingChatRoutingValueBuilder {
    channel_id: Option<String>,
    channel_name: Option<String>,
}

impl QueryIntegrationsSlackInstallationResponseProviderMetadataChatRoutingChatRoutingValueBuilder {
    pub fn channel_id(mut self, value: impl Into<String>) -> Self {
        self.channel_id = Some(value.into());
        self
    }

    pub fn channel_name(mut self, value: impl Into<String>) -> Self {
        self.channel_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsSlackInstallationResponseProviderMetadataChatRoutingChatRoutingValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`channel_id`](QueryIntegrationsSlackInstallationResponseProviderMetadataChatRoutingChatRoutingValueBuilder::channel_id)
    /// - [`channel_name`](QueryIntegrationsSlackInstallationResponseProviderMetadataChatRoutingChatRoutingValueBuilder::channel_name)
    pub fn build(self) -> Result<QueryIntegrationsSlackInstallationResponseProviderMetadataChatRoutingChatRoutingValue, BuildError> {
        Ok(QueryIntegrationsSlackInstallationResponseProviderMetadataChatRoutingChatRoutingValue {
            channel_id: self.channel_id.ok_or_else(|| BuildError::missing_field("channel_id"))?,
            channel_name: self.channel_name.ok_or_else(|| BuildError::missing_field("channel_name"))?,
        })
    }
}
