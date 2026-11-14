pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimelineEventContentSixDocContentItemOneAttrsAnnotationsItem {
    #[serde(default)]
    pub line: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<TimelineEventContentSixDocContentItemOneAttrsAnnotationsItemSide>,
    #[serde(default)]
    pub text: String,
}

impl TimelineEventContentSixDocContentItemOneAttrsAnnotationsItem {
    pub fn builder() -> TimelineEventContentSixDocContentItemOneAttrsAnnotationsItemBuilder {
        <TimelineEventContentSixDocContentItemOneAttrsAnnotationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentSixDocContentItemOneAttrsAnnotationsItemBuilder {
    line: Option<i64>,
    side: Option<TimelineEventContentSixDocContentItemOneAttrsAnnotationsItemSide>,
    text: Option<String>,
}

impl TimelineEventContentSixDocContentItemOneAttrsAnnotationsItemBuilder {
    pub fn line(mut self, value: i64) -> Self {
        self.line = Some(value);
        self
    }

    pub fn side(mut self, value: TimelineEventContentSixDocContentItemOneAttrsAnnotationsItemSide) -> Self {
        self.side = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentSixDocContentItemOneAttrsAnnotationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`line`](TimelineEventContentSixDocContentItemOneAttrsAnnotationsItemBuilder::line)
    /// - [`text`](TimelineEventContentSixDocContentItemOneAttrsAnnotationsItemBuilder::text)
    pub fn build(self) -> Result<TimelineEventContentSixDocContentItemOneAttrsAnnotationsItem, BuildError> {
        Ok(TimelineEventContentSixDocContentItemOneAttrsAnnotationsItem {
            line: self.line.ok_or_else(|| BuildError::missing_field("line"))?,
            side: self.side,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
