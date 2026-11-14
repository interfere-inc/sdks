pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimelineEventContentChartDocContentItemOneAttrsAnnotationsItem {
    #[serde(default)]
    pub line: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<TimelineEventContentChartDocContentItemOneAttrsAnnotationsItemSide>,
    #[serde(default)]
    pub text: String,
}

impl TimelineEventContentChartDocContentItemOneAttrsAnnotationsItem {
    pub fn builder() -> TimelineEventContentChartDocContentItemOneAttrsAnnotationsItemBuilder {
        <TimelineEventContentChartDocContentItemOneAttrsAnnotationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartDocContentItemOneAttrsAnnotationsItemBuilder {
    line: Option<i64>,
    side: Option<TimelineEventContentChartDocContentItemOneAttrsAnnotationsItemSide>,
    text: Option<String>,
}

impl TimelineEventContentChartDocContentItemOneAttrsAnnotationsItemBuilder {
    pub fn line(mut self, value: i64) -> Self {
        self.line = Some(value);
        self
    }

    pub fn side(mut self, value: TimelineEventContentChartDocContentItemOneAttrsAnnotationsItemSide) -> Self {
        self.side = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentChartDocContentItemOneAttrsAnnotationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`line`](TimelineEventContentChartDocContentItemOneAttrsAnnotationsItemBuilder::line)
    /// - [`text`](TimelineEventContentChartDocContentItemOneAttrsAnnotationsItemBuilder::text)
    pub fn build(self) -> Result<TimelineEventContentChartDocContentItemOneAttrsAnnotationsItem, BuildError> {
        Ok(TimelineEventContentChartDocContentItemOneAttrsAnnotationsItem {
            line: self.line.ok_or_else(|| BuildError::missing_field("line"))?,
            side: self.side,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
