pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationTimelineEventsToggleReactionResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationTimelineEventsToggleReactionResponse {
    pub fn builder() -> MutationTimelineEventsToggleReactionResponseBuilder {
        <MutationTimelineEventsToggleReactionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationTimelineEventsToggleReactionResponseBuilder {
    success: Option<bool>,
}

impl MutationTimelineEventsToggleReactionResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationTimelineEventsToggleReactionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationTimelineEventsToggleReactionResponseBuilder::success)
    pub fn build(self) -> Result<MutationTimelineEventsToggleReactionResponse, BuildError> {
        Ok(MutationTimelineEventsToggleReactionResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
