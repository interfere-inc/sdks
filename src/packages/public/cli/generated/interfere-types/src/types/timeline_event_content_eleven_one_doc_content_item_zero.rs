pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentElevenOneDocContentItemZero {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TimelineEventContentElevenOneDocContentItemZeroContentItem>>,
    pub r#type: TimelineEventContentElevenOneDocContentItemZeroType,
}

impl TimelineEventContentElevenOneDocContentItemZero {
    pub fn builder() -> TimelineEventContentElevenOneDocContentItemZeroBuilder {
        <TimelineEventContentElevenOneDocContentItemZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentElevenOneDocContentItemZeroBuilder {
    content: Option<Vec<TimelineEventContentElevenOneDocContentItemZeroContentItem>>,
    r#type: Option<TimelineEventContentElevenOneDocContentItemZeroType>,
}

impl TimelineEventContentElevenOneDocContentItemZeroBuilder {
    pub fn content(mut self, value: Vec<TimelineEventContentElevenOneDocContentItemZeroContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentElevenOneDocContentItemZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentElevenOneDocContentItemZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentElevenOneDocContentItemZeroBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentElevenOneDocContentItemZero, BuildError> {
        Ok(TimelineEventContentElevenOneDocContentItemZero {
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
