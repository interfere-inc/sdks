pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentChartChartSlices {
    #[serde(default)]
    pub slices: Vec<TimelineEventContentChartChartSlicesSlicesItem>,
    pub r#type: TimelineEventContentChartChartSlicesType,
}

impl TimelineEventContentChartChartSlices {
    pub fn builder() -> TimelineEventContentChartChartSlicesBuilder {
        <TimelineEventContentChartChartSlicesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartChartSlicesBuilder {
    slices: Option<Vec<TimelineEventContentChartChartSlicesSlicesItem>>,
    r#type: Option<TimelineEventContentChartChartSlicesType>,
}

impl TimelineEventContentChartChartSlicesBuilder {
    pub fn slices(mut self, value: Vec<TimelineEventContentChartChartSlicesSlicesItem>) -> Self {
        self.slices = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentChartChartSlicesType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentChartChartSlices`].
    /// This method will fail if any of the following fields are not set:
    /// - [`slices`](TimelineEventContentChartChartSlicesBuilder::slices)
    /// - [`r#type`](TimelineEventContentChartChartSlicesBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentChartChartSlices, BuildError> {
        Ok(TimelineEventContentChartChartSlices {
            slices: self.slices.ok_or_else(|| BuildError::missing_field("slices"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
