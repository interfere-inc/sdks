pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationNotificationPreferencesSetWorkflowChannelResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationNotificationPreferencesSetWorkflowChannelResponse {
    pub fn builder() -> MutationNotificationPreferencesSetWorkflowChannelResponseBuilder {
        <MutationNotificationPreferencesSetWorkflowChannelResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationNotificationPreferencesSetWorkflowChannelResponseBuilder {
    success: Option<bool>,
}

impl MutationNotificationPreferencesSetWorkflowChannelResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationNotificationPreferencesSetWorkflowChannelResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationNotificationPreferencesSetWorkflowChannelResponseBuilder::success)
    pub fn build(self) -> Result<MutationNotificationPreferencesSetWorkflowChannelResponse, BuildError> {
        Ok(MutationNotificationPreferencesSetWorkflowChannelResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
