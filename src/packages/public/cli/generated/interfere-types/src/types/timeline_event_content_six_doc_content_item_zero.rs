pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentSixDocContentItemZero {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TimelineEventContentSixDocContentItemZeroContentItem>>,
    pub r#type: TimelineEventContentSixDocContentItemZeroType,
}

impl TimelineEventContentSixDocContentItemZero {
    pub fn builder() -> TimelineEventContentSixDocContentItemZeroBuilder {
        <TimelineEventContentSixDocContentItemZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentSixDocContentItemZeroBuilder {
    content: Option<Vec<TimelineEventContentSixDocContentItemZeroContentItem>>,
    r#type: Option<TimelineEventContentSixDocContentItemZeroType>,
}

impl TimelineEventContentSixDocContentItemZeroBuilder {
    pub fn content(mut self, value: Vec<TimelineEventContentSixDocContentItemZeroContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentSixDocContentItemZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentSixDocContentItemZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentSixDocContentItemZeroBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentSixDocContentItemZero, BuildError> {
        Ok(TimelineEventContentSixDocContentItemZero {
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
