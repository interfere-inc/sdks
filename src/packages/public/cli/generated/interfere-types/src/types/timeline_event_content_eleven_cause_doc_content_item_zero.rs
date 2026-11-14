pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentElevenCauseDocContentItemZero {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TimelineEventContentElevenCauseDocContentItemZeroContentItem>>,
    pub r#type: TimelineEventContentElevenCauseDocContentItemZeroType,
}

impl TimelineEventContentElevenCauseDocContentItemZero {
    pub fn builder() -> TimelineEventContentElevenCauseDocContentItemZeroBuilder {
        <TimelineEventContentElevenCauseDocContentItemZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentElevenCauseDocContentItemZeroBuilder {
    content: Option<Vec<TimelineEventContentElevenCauseDocContentItemZeroContentItem>>,
    r#type: Option<TimelineEventContentElevenCauseDocContentItemZeroType>,
}

impl TimelineEventContentElevenCauseDocContentItemZeroBuilder {
    pub fn content(mut self, value: Vec<TimelineEventContentElevenCauseDocContentItemZeroContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentElevenCauseDocContentItemZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentElevenCauseDocContentItemZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentElevenCauseDocContentItemZeroBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentElevenCauseDocContentItemZero, BuildError> {
        Ok(TimelineEventContentElevenCauseDocContentItemZero {
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
