pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSlackChannelsResponse {
    #[serde(default)]
    pub channels: Vec<ListSlackChannelsResponseChannelsItem>,
}

impl ListSlackChannelsResponse {
    pub fn builder() -> ListSlackChannelsResponseBuilder {
        <ListSlackChannelsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSlackChannelsResponseBuilder {
    channels: Option<Vec<ListSlackChannelsResponseChannelsItem>>,
}

impl ListSlackChannelsResponseBuilder {
    pub fn channels(mut self, value: Vec<ListSlackChannelsResponseChannelsItem>) -> Self {
        self.channels = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSlackChannelsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`channels`](ListSlackChannelsResponseBuilder::channels)
    pub fn build(self) -> Result<ListSlackChannelsResponse, BuildError> {
        Ok(ListSlackChannelsResponse {
            channels: self.channels.ok_or_else(|| BuildError::missing_field("channels"))?,
        })
    }
}
