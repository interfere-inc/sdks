pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemZero {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<Option<serde_json::Value>>>,
    pub r#type: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemZeroType,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemZero {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemZeroBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemZeroBuilder {
    content: Option<Vec<Option<serde_json::Value>>>,
    r#type: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemZeroType>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemZeroBuilder {
    pub fn content(mut self, value: Vec<Option<serde_json::Value>>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemZeroBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemZero, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemZero {
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
