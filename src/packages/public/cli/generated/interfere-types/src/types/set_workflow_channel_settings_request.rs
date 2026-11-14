pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SetWorkflowChannelSettingsRequest {
    pub args: SetWorkflowChannelSettingsRequestArgs,
}

impl SetWorkflowChannelSettingsRequest {
    pub fn builder() -> SetWorkflowChannelSettingsRequestBuilder {
        <SetWorkflowChannelSettingsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetWorkflowChannelSettingsRequestBuilder {
    args: Option<SetWorkflowChannelSettingsRequestArgs>,
}

impl SetWorkflowChannelSettingsRequestBuilder {
    pub fn args(mut self, value: SetWorkflowChannelSettingsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetWorkflowChannelSettingsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](SetWorkflowChannelSettingsRequestBuilder::args)
    pub fn build(self) -> Result<SetWorkflowChannelSettingsRequest, BuildError> {
        Ok(SetWorkflowChannelSettingsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

