pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSlackChannelsResponseChannelsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "isPrivate")]
    #[serde(default)]
    pub is_private: bool,
}

impl ListSlackChannelsResponseChannelsItem {
    pub fn builder() -> ListSlackChannelsResponseChannelsItemBuilder {
        <ListSlackChannelsResponseChannelsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSlackChannelsResponseChannelsItemBuilder {
    id: Option<String>,
    name: Option<String>,
    is_private: Option<bool>,
}

impl ListSlackChannelsResponseChannelsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn is_private(mut self, value: bool) -> Self {
        self.is_private = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSlackChannelsResponseChannelsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ListSlackChannelsResponseChannelsItemBuilder::id)
    /// - [`name`](ListSlackChannelsResponseChannelsItemBuilder::name)
    /// - [`is_private`](ListSlackChannelsResponseChannelsItemBuilder::is_private)
    pub fn build(self) -> Result<ListSlackChannelsResponseChannelsItem, BuildError> {
        Ok(ListSlackChannelsResponseChannelsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            is_private: self.is_private.ok_or_else(|| BuildError::missing_field("is_private"))?,
        })
    }
}
