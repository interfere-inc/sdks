pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectionProviderMetadataChatRoutingChatRoutingValue {
    #[serde(rename = "channelId")]
    #[serde(default)]
    pub channel_id: String,
    #[serde(rename = "channelName")]
    #[serde(default)]
    pub channel_name: String,
}

impl ConnectionProviderMetadataChatRoutingChatRoutingValue {
    pub fn builder() -> ConnectionProviderMetadataChatRoutingChatRoutingValueBuilder {
        <ConnectionProviderMetadataChatRoutingChatRoutingValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionProviderMetadataChatRoutingChatRoutingValueBuilder {
    channel_id: Option<String>,
    channel_name: Option<String>,
}

impl ConnectionProviderMetadataChatRoutingChatRoutingValueBuilder {
    pub fn channel_id(mut self, value: impl Into<String>) -> Self {
        self.channel_id = Some(value.into());
        self
    }

    pub fn channel_name(mut self, value: impl Into<String>) -> Self {
        self.channel_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConnectionProviderMetadataChatRoutingChatRoutingValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`channel_id`](ConnectionProviderMetadataChatRoutingChatRoutingValueBuilder::channel_id)
    /// - [`channel_name`](ConnectionProviderMetadataChatRoutingChatRoutingValueBuilder::channel_name)
    pub fn build(self) -> Result<ConnectionProviderMetadataChatRoutingChatRoutingValue, BuildError> {
        Ok(ConnectionProviderMetadataChatRoutingChatRoutingValue {
            channel_id: self.channel_id.ok_or_else(|| BuildError::missing_field("channel_id"))?,
            channel_name: self.channel_name.ok_or_else(|| BuildError::missing_field("channel_name"))?,
        })
    }
}
