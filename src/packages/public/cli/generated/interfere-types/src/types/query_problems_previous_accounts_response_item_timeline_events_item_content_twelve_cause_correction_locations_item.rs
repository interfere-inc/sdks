pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItem {
    #[serde(default)]
    pub repository: String,
    #[serde(default)]
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItemLines>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItem {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItemBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItemBuilder {
    repository: Option<String>,
    path: Option<String>,
    lines: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItemLines>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItemBuilder {
    pub fn repository(mut self, value: impl Into<String>) -> Self {
        self.repository = Some(value.into());
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn lines(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItemLines) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`repository`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItemBuilder::repository)
    /// - [`path`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItemBuilder::path)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItem, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItem {
            repository: self.repository.ok_or_else(|| BuildError::missing_field("repository"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            lines: self.lines,
        })
    }
}
