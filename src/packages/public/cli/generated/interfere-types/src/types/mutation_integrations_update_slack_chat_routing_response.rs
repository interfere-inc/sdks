pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationIntegrationsUpdateSlackChatRoutingResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationIntegrationsUpdateSlackChatRoutingResponse {
    pub fn builder() -> MutationIntegrationsUpdateSlackChatRoutingResponseBuilder {
        <MutationIntegrationsUpdateSlackChatRoutingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationIntegrationsUpdateSlackChatRoutingResponseBuilder {
    success: Option<bool>,
}

impl MutationIntegrationsUpdateSlackChatRoutingResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationIntegrationsUpdateSlackChatRoutingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationIntegrationsUpdateSlackChatRoutingResponseBuilder::success)
    pub fn build(self) -> Result<MutationIntegrationsUpdateSlackChatRoutingResponse, BuildError> {
        Ok(MutationIntegrationsUpdateSlackChatRoutingResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
