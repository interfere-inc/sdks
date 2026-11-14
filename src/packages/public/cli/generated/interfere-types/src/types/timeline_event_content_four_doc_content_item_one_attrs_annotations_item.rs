pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimelineEventContentFourDocContentItemOneAttrsAnnotationsItem {
    #[serde(default)]
    pub line: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<TimelineEventContentFourDocContentItemOneAttrsAnnotationsItemSide>,
    #[serde(default)]
    pub text: String,
}

impl TimelineEventContentFourDocContentItemOneAttrsAnnotationsItem {
    pub fn builder() -> TimelineEventContentFourDocContentItemOneAttrsAnnotationsItemBuilder {
        <TimelineEventContentFourDocContentItemOneAttrsAnnotationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentFourDocContentItemOneAttrsAnnotationsItemBuilder {
    line: Option<i64>,
    side: Option<TimelineEventContentFourDocContentItemOneAttrsAnnotationsItemSide>,
    text: Option<String>,
}

impl TimelineEventContentFourDocContentItemOneAttrsAnnotationsItemBuilder {
    pub fn line(mut self, value: i64) -> Self {
        self.line = Some(value);
        self
    }

    pub fn side(mut self, value: TimelineEventContentFourDocContentItemOneAttrsAnnotationsItemSide) -> Self {
        self.side = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentFourDocContentItemOneAttrsAnnotationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`line`](TimelineEventContentFourDocContentItemOneAttrsAnnotationsItemBuilder::line)
    /// - [`text`](TimelineEventContentFourDocContentItemOneAttrsAnnotationsItemBuilder::text)
    pub fn build(self) -> Result<TimelineEventContentFourDocContentItemOneAttrsAnnotationsItem, BuildError> {
        Ok(TimelineEventContentFourDocContentItemOneAttrsAnnotationsItem {
            line: self.line.ok_or_else(|| BuildError::missing_field("line"))?,
            side: self.side,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
