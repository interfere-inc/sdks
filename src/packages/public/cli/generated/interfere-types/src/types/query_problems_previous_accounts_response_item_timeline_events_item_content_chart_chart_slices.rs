pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlices {
    #[serde(default)]
    pub slices: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlicesSlicesItem>,
    pub r#type: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlicesType,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlices {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlicesBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlicesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlicesBuilder {
    slices: Option<Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlicesSlicesItem>>,
    r#type: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlicesType>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlicesBuilder {
    pub fn slices(mut self, value: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlicesSlicesItem>) -> Self {
        self.slices = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlicesType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlices`].
    /// This method will fail if any of the following fields are not set:
    /// - [`slices`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlicesBuilder::slices)
    /// - [`r#type`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlicesBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlices, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlices {
            slices: self.slices.ok_or_else(|| BuildError::missing_field("slices"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
