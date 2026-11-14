pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisMarkersItem {
    #[serde(default)]
    pub index: i64,
    #[serde(default)]
    pub label: String,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisMarkersItem {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisMarkersItemBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisMarkersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisMarkersItemBuilder {
    index: Option<i64>,
    label: Option<String>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisMarkersItemBuilder {
    pub fn index(mut self, value: i64) -> Self {
        self.index = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisMarkersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`index`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisMarkersItemBuilder::index)
    /// - [`label`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisMarkersItemBuilder::label)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisMarkersItem, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisMarkersItem {
            index: self.index.ok_or_else(|| BuildError::missing_field("index"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
        })
    }
}
