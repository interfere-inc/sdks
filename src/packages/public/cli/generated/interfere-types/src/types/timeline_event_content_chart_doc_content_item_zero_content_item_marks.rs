pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentChartDocContentItemZeroContentItemMarks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marks: Option<Vec<Option<serde_json::Value>>>,
    #[serde(default)]
    pub text: String,
    pub r#type: TimelineEventContentChartDocContentItemZeroContentItemMarksType,
}

impl TimelineEventContentChartDocContentItemZeroContentItemMarks {
    pub fn builder() -> TimelineEventContentChartDocContentItemZeroContentItemMarksBuilder {
        <TimelineEventContentChartDocContentItemZeroContentItemMarksBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartDocContentItemZeroContentItemMarksBuilder {
    marks: Option<Vec<Option<serde_json::Value>>>,
    text: Option<String>,
    r#type: Option<TimelineEventContentChartDocContentItemZeroContentItemMarksType>,
}

impl TimelineEventContentChartDocContentItemZeroContentItemMarksBuilder {
    pub fn marks(mut self, value: Vec<Option<serde_json::Value>>) -> Self {
        self.marks = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentChartDocContentItemZeroContentItemMarksType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentChartDocContentItemZeroContentItemMarks`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](TimelineEventContentChartDocContentItemZeroContentItemMarksBuilder::text)
    /// - [`r#type`](TimelineEventContentChartDocContentItemZeroContentItemMarksBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentChartDocContentItemZeroContentItemMarks, BuildError> {
        Ok(TimelineEventContentChartDocContentItemZeroContentItemMarks {
            marks: self.marks,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
