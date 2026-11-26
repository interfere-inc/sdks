pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentTwelveOneDocContentItemZero {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TimelineEventContentTwelveOneDocContentItemZeroContentItem>>,
    pub r#type: TimelineEventContentTwelveOneDocContentItemZeroType,
}

impl TimelineEventContentTwelveOneDocContentItemZero {
    pub fn builder() -> TimelineEventContentTwelveOneDocContentItemZeroBuilder {
        <TimelineEventContentTwelveOneDocContentItemZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentTwelveOneDocContentItemZeroBuilder {
    content: Option<Vec<TimelineEventContentTwelveOneDocContentItemZeroContentItem>>,
    r#type: Option<TimelineEventContentTwelveOneDocContentItemZeroType>,
}

impl TimelineEventContentTwelveOneDocContentItemZeroBuilder {
    pub fn content(mut self, value: Vec<TimelineEventContentTwelveOneDocContentItemZeroContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentTwelveOneDocContentItemZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentTwelveOneDocContentItemZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentTwelveOneDocContentItemZeroBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentTwelveOneDocContentItemZero, BuildError> {
        Ok(TimelineEventContentTwelveOneDocContentItemZero {
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
