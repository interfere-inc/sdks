pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlices {
    #[serde(default)]
    pub slices: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlicesSlicesItem>,
    pub r#type: QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlicesType,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlices {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlicesBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlicesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlicesBuilder {
    slices: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlicesSlicesItem>>,
    r#type: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlicesType>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlicesBuilder {
    pub fn slices(mut self, value: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlicesSlicesItem>) -> Self {
        self.slices = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlicesType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlices`].
    /// This method will fail if any of the following fields are not set:
    /// - [`slices`](QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlicesBuilder::slices)
    /// - [`r#type`](QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlicesBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlices, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlices {
            slices: self.slices.ok_or_else(|| BuildError::missing_field("slices"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
