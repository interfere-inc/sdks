pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDoc {
    #[serde(default)]
    pub content: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItem>,
    pub r#type: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocType,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDoc {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocBuilder {
    content: Option<Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItem>>,
    r#type: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocType>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocBuilder {
    pub fn content(mut self, value: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDoc`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocBuilder::content)
    /// - [`r#type`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDoc, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDoc {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
