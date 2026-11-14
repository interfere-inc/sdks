pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ToggleReactionTimelineRequestArgs {
    #[serde(rename = "reactionId")]
    #[serde(default)]
    pub reaction_id: String,
    #[serde(rename = "timelineEventId")]
    #[serde(default)]
    pub timeline_event_id: String,
    #[serde(default)]
    pub emoji: String,
}

impl ToggleReactionTimelineRequestArgs {
    pub fn builder() -> ToggleReactionTimelineRequestArgsBuilder {
        <ToggleReactionTimelineRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToggleReactionTimelineRequestArgsBuilder {
    reaction_id: Option<String>,
    timeline_event_id: Option<String>,
    emoji: Option<String>,
}

impl ToggleReactionTimelineRequestArgsBuilder {
    pub fn reaction_id(mut self, value: impl Into<String>) -> Self {
        self.reaction_id = Some(value.into());
        self
    }

    pub fn timeline_event_id(mut self, value: impl Into<String>) -> Self {
        self.timeline_event_id = Some(value.into());
        self
    }

    pub fn emoji(mut self, value: impl Into<String>) -> Self {
        self.emoji = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ToggleReactionTimelineRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reaction_id`](ToggleReactionTimelineRequestArgsBuilder::reaction_id)
    /// - [`timeline_event_id`](ToggleReactionTimelineRequestArgsBuilder::timeline_event_id)
    /// - [`emoji`](ToggleReactionTimelineRequestArgsBuilder::emoji)
    pub fn build(self) -> Result<ToggleReactionTimelineRequestArgs, BuildError> {
        Ok(ToggleReactionTimelineRequestArgs {
            reaction_id: self.reaction_id.ok_or_else(|| BuildError::missing_field("reaction_id"))?,
            timeline_event_id: self.timeline_event_id.ok_or_else(|| BuildError::missing_field("timeline_event_id"))?,
            emoji: self.emoji.ok_or_else(|| BuildError::missing_field("emoji"))?,
        })
    }
}
