pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateSlackChatRoutingIntegrationsRequestArgs {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    pub category: UpdateSlackChatRoutingIntegrationsRequestArgsCategory,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<UpdateSlackChatRoutingIntegrationsRequestArgsEntry>,
}

impl UpdateSlackChatRoutingIntegrationsRequestArgs {
    pub fn builder() -> UpdateSlackChatRoutingIntegrationsRequestArgsBuilder {
        <UpdateSlackChatRoutingIntegrationsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateSlackChatRoutingIntegrationsRequestArgsBuilder {
    connection_id: Option<String>,
    category: Option<UpdateSlackChatRoutingIntegrationsRequestArgsCategory>,
    entry: Option<UpdateSlackChatRoutingIntegrationsRequestArgsEntry>,
}

impl UpdateSlackChatRoutingIntegrationsRequestArgsBuilder {
    pub fn connection_id(mut self, value: impl Into<String>) -> Self {
        self.connection_id = Some(value.into());
        self
    }

    pub fn category(mut self, value: UpdateSlackChatRoutingIntegrationsRequestArgsCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn entry(mut self, value: UpdateSlackChatRoutingIntegrationsRequestArgsEntry) -> Self {
        self.entry = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateSlackChatRoutingIntegrationsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connection_id`](UpdateSlackChatRoutingIntegrationsRequestArgsBuilder::connection_id)
    /// - [`category`](UpdateSlackChatRoutingIntegrationsRequestArgsBuilder::category)
    pub fn build(self) -> Result<UpdateSlackChatRoutingIntegrationsRequestArgs, BuildError> {
        Ok(UpdateSlackChatRoutingIntegrationsRequestArgs {
            connection_id: self.connection_id.ok_or_else(|| BuildError::missing_field("connection_id"))?,
            category: self.category.ok_or_else(|| BuildError::missing_field("category"))?,
            entry: self.entry,
        })
    }
}
