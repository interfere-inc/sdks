pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateSlackChatRoutingIntegrationsRequest {
    pub args: UpdateSlackChatRoutingIntegrationsRequestArgs,
}

impl UpdateSlackChatRoutingIntegrationsRequest {
    pub fn builder() -> UpdateSlackChatRoutingIntegrationsRequestBuilder {
        <UpdateSlackChatRoutingIntegrationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateSlackChatRoutingIntegrationsRequestBuilder {
    args: Option<UpdateSlackChatRoutingIntegrationsRequestArgs>,
}

impl UpdateSlackChatRoutingIntegrationsRequestBuilder {
    pub fn args(mut self, value: UpdateSlackChatRoutingIntegrationsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateSlackChatRoutingIntegrationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UpdateSlackChatRoutingIntegrationsRequestBuilder::args)
    pub fn build(self) -> Result<UpdateSlackChatRoutingIntegrationsRequest, BuildError> {
        Ok(UpdateSlackChatRoutingIntegrationsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

