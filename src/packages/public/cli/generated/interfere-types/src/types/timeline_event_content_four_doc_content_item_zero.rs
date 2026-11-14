pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentFourDocContentItemZero {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TimelineEventContentFourDocContentItemZeroContentItem>>,
    pub r#type: TimelineEventContentFourDocContentItemZeroType,
}

impl TimelineEventContentFourDocContentItemZero {
    pub fn builder() -> TimelineEventContentFourDocContentItemZeroBuilder {
        <TimelineEventContentFourDocContentItemZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentFourDocContentItemZeroBuilder {
    content: Option<Vec<TimelineEventContentFourDocContentItemZeroContentItem>>,
    r#type: Option<TimelineEventContentFourDocContentItemZeroType>,
}

impl TimelineEventContentFourDocContentItemZeroBuilder {
    pub fn content(mut self, value: Vec<TimelineEventContentFourDocContentItemZeroContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentFourDocContentItemZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentFourDocContentItemZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentFourDocContentItemZeroBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentFourDocContentItemZero, BuildError> {
        Ok(TimelineEventContentFourDocContentItemZero {
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
