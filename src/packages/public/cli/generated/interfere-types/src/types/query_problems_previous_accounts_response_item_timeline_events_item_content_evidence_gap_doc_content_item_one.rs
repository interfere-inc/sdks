pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOne {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOneAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<Option<serde_json::Value>>>,
    pub r#type: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOneType,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOne {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOneBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOneBuilder {
    attrs: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOneAttrs>,
    content: Option<Vec<Option<serde_json::Value>>>,
    r#type: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOneType>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOneBuilder {
    pub fn attrs(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOneAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn content(mut self, value: Vec<Option<serde_json::Value>>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOneBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOne, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOne {
            attrs: self.attrs,
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
