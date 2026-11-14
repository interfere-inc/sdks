pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDoc {
    #[serde(default)]
    pub content: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItem>,
    pub r#type: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocType,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDoc {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocBuilder {
    content: Option<Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItem>>,
    r#type: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocType>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocBuilder {
    pub fn content(mut self, value: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDoc`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocBuilder::content)
    /// - [`r#type`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDoc, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDoc {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
