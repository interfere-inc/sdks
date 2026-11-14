pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDoc {
    #[serde(default)]
    pub content: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItem>,
    pub r#type: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocType,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDoc {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocBuilder {
    content: Option<Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItem>>,
    r#type: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocType>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocBuilder {
    pub fn content(mut self, value: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDoc`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocBuilder::content)
    /// - [`r#type`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDoc, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDoc {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
