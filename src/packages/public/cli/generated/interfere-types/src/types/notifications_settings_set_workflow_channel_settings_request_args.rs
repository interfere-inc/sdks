pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SetWorkflowChannelSettingsRequestArgs {
    #[serde(rename = "preferenceId")]
    #[serde(default)]
    pub preference_id: String,
    #[serde(rename = "workflowKeys")]
    #[serde(default)]
    pub workflow_keys: Vec<SetWorkflowChannelSettingsRequestArgsWorkflowKeysItem>,
    pub channel: SetWorkflowChannelSettingsRequestArgsChannel,
    #[serde(default)]
    pub enabled: bool,
}

impl SetWorkflowChannelSettingsRequestArgs {
    pub fn builder() -> SetWorkflowChannelSettingsRequestArgsBuilder {
        <SetWorkflowChannelSettingsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetWorkflowChannelSettingsRequestArgsBuilder {
    preference_id: Option<String>,
    workflow_keys: Option<Vec<SetWorkflowChannelSettingsRequestArgsWorkflowKeysItem>>,
    channel: Option<SetWorkflowChannelSettingsRequestArgsChannel>,
    enabled: Option<bool>,
}

impl SetWorkflowChannelSettingsRequestArgsBuilder {
    pub fn preference_id(mut self, value: impl Into<String>) -> Self {
        self.preference_id = Some(value.into());
        self
    }

    pub fn workflow_keys(mut self, value: Vec<SetWorkflowChannelSettingsRequestArgsWorkflowKeysItem>) -> Self {
        self.workflow_keys = Some(value);
        self
    }

    pub fn channel(mut self, value: SetWorkflowChannelSettingsRequestArgsChannel) -> Self {
        self.channel = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetWorkflowChannelSettingsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`preference_id`](SetWorkflowChannelSettingsRequestArgsBuilder::preference_id)
    /// - [`workflow_keys`](SetWorkflowChannelSettingsRequestArgsBuilder::workflow_keys)
    /// - [`channel`](SetWorkflowChannelSettingsRequestArgsBuilder::channel)
    /// - [`enabled`](SetWorkflowChannelSettingsRequestArgsBuilder::enabled)
    pub fn build(self) -> Result<SetWorkflowChannelSettingsRequestArgs, BuildError> {
        Ok(SetWorkflowChannelSettingsRequestArgs {
            preference_id: self.preference_id.ok_or_else(|| BuildError::missing_field("preference_id"))?,
            workflow_keys: self.workflow_keys.ok_or_else(|| BuildError::missing_field("workflow_keys"))?,
            channel: self.channel.ok_or_else(|| BuildError::missing_field("channel"))?,
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
        })
    }
}
