pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemZero {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<Option<serde_json::Value>>>,
    pub r#type: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemZeroType,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemZero {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemZeroBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemZeroBuilder {
    content: Option<Vec<Option<serde_json::Value>>>,
    r#type: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemZeroType>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemZeroBuilder {
    pub fn content(mut self, value: Vec<Option<serde_json::Value>>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemZeroBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemZero, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemZero {
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
