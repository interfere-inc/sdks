pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimelineEventContentChartChartAxisAxis {
    #[serde(default)]
    pub end: String,
    #[serde(default)]
    pub start: String,
}

impl TimelineEventContentChartChartAxisAxis {
    pub fn builder() -> TimelineEventContentChartChartAxisAxisBuilder {
        <TimelineEventContentChartChartAxisAxisBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartChartAxisAxisBuilder {
    end: Option<String>,
    start: Option<String>,
}

impl TimelineEventContentChartChartAxisAxisBuilder {
    pub fn end(mut self, value: impl Into<String>) -> Self {
        self.end = Some(value.into());
        self
    }

    pub fn start(mut self, value: impl Into<String>) -> Self {
        self.start = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentChartChartAxisAxis`].
    /// This method will fail if any of the following fields are not set:
    /// - [`end`](TimelineEventContentChartChartAxisAxisBuilder::end)
    /// - [`start`](TimelineEventContentChartChartAxisAxisBuilder::start)
    pub fn build(self) -> Result<TimelineEventContentChartChartAxisAxis, BuildError> {
        Ok(TimelineEventContentChartChartAxisAxis {
            end: self.end.ok_or_else(|| BuildError::missing_field("end"))?,
            start: self.start.ok_or_else(|| BuildError::missing_field("start"))?,
        })
    }
}
