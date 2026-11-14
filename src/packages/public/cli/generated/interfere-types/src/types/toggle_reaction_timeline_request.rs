pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ToggleReactionTimelineRequest {
    #[serde(default)]
    pub args: ToggleReactionTimelineRequestArgs,
}

impl ToggleReactionTimelineRequest {
    pub fn builder() -> ToggleReactionTimelineRequestBuilder {
        <ToggleReactionTimelineRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToggleReactionTimelineRequestBuilder {
    args: Option<ToggleReactionTimelineRequestArgs>,
}

impl ToggleReactionTimelineRequestBuilder {
    pub fn args(mut self, value: ToggleReactionTimelineRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToggleReactionTimelineRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](ToggleReactionTimelineRequestBuilder::args)
    pub fn build(self) -> Result<ToggleReactionTimelineRequest, BuildError> {
        Ok(ToggleReactionTimelineRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

