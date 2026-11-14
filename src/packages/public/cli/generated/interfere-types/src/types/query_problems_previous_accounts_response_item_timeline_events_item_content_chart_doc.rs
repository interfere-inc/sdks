pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDoc {
    #[serde(default)]
    pub content: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItem>,
    pub r#type: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocType,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDoc {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocBuilder {
    content: Option<Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItem>>,
    r#type: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocType>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocBuilder {
    pub fn content(mut self, value: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDoc`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocBuilder::content)
    /// - [`r#type`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDoc, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDoc {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
