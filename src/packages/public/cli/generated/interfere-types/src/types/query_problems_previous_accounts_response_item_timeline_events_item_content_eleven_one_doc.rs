pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDoc {
    #[serde(default)]
    pub content: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItem>,
    pub r#type: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocType,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDoc {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocBuilder {
    content: Option<Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItem>>,
    r#type: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocType>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocBuilder {
    pub fn content(mut self, value: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDoc`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocBuilder::content)
    /// - [`r#type`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDoc, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDoc {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
