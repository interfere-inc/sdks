pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateSlackChatRoutingIntegrationsRequestArgsEntry {
    #[serde(rename = "channelId")]
    #[serde(default)]
    pub channel_id: String,
    #[serde(rename = "channelName")]
    #[serde(default)]
    pub channel_name: String,
}

impl UpdateSlackChatRoutingIntegrationsRequestArgsEntry {
    pub fn builder() -> UpdateSlackChatRoutingIntegrationsRequestArgsEntryBuilder {
        <UpdateSlackChatRoutingIntegrationsRequestArgsEntryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateSlackChatRoutingIntegrationsRequestArgsEntryBuilder {
    channel_id: Option<String>,
    channel_name: Option<String>,
}

impl UpdateSlackChatRoutingIntegrationsRequestArgsEntryBuilder {
    pub fn channel_id(mut self, value: impl Into<String>) -> Self {
        self.channel_id = Some(value.into());
        self
    }

    pub fn channel_name(mut self, value: impl Into<String>) -> Self {
        self.channel_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateSlackChatRoutingIntegrationsRequestArgsEntry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`channel_id`](UpdateSlackChatRoutingIntegrationsRequestArgsEntryBuilder::channel_id)
    /// - [`channel_name`](UpdateSlackChatRoutingIntegrationsRequestArgsEntryBuilder::channel_name)
    pub fn build(self) -> Result<UpdateSlackChatRoutingIntegrationsRequestArgsEntry, BuildError> {
        Ok(UpdateSlackChatRoutingIntegrationsRequestArgsEntry {
            channel_id: self.channel_id.ok_or_else(|| BuildError::missing_field("channel_id"))?,
            channel_name: self.channel_name.ok_or_else(|| BuildError::missing_field("channel_name"))?,
        })
    }
}
