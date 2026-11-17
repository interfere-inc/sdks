pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingChatRoutingValue {
    #[serde(rename = "channelId")]
    #[serde(default)]
    pub channel_id: String,
    #[serde(rename = "channelName")]
    #[serde(default)]
    pub channel_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingChatRoutingValueThreshold>,
}

impl QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingChatRoutingValue {
    pub fn builder() -> QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingChatRoutingValueBuilder {
        <QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingChatRoutingValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingChatRoutingValueBuilder {
    channel_id: Option<String>,
    channel_name: Option<String>,
    threshold: Option<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingChatRoutingValueThreshold>,
}

impl QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingChatRoutingValueBuilder {
    pub fn channel_id(mut self, value: impl Into<String>) -> Self {
        self.channel_id = Some(value.into());
        self
    }

    pub fn channel_name(mut self, value: impl Into<String>) -> Self {
        self.channel_name = Some(value.into());
        self
    }

    pub fn threshold(mut self, value: QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingChatRoutingValueThreshold) -> Self {
        self.threshold = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingChatRoutingValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`channel_id`](QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingChatRoutingValueBuilder::channel_id)
    /// - [`channel_name`](QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingChatRoutingValueBuilder::channel_name)
    pub fn build(self) -> Result<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingChatRoutingValue, BuildError> {
        Ok(QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataChatRoutingChatRoutingValue {
            channel_id: self.channel_id.ok_or_else(|| BuildError::missing_field("channel_id"))?,
            channel_name: self.channel_name.ok_or_else(|| BuildError::missing_field("channel_name"))?,
            threshold: self.threshold,
        })
    }
}
