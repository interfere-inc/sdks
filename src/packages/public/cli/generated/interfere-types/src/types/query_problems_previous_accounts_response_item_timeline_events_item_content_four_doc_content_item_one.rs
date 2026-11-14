pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemOne {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemOneAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<Option<serde_json::Value>>>,
    pub r#type: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemOneType,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemOne {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemOneBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemOneBuilder {
    attrs: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemOneAttrs>,
    content: Option<Vec<Option<serde_json::Value>>>,
    r#type: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemOneType>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemOneBuilder {
    pub fn attrs(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemOneAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn content(mut self, value: Vec<Option<serde_json::Value>>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemOneBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemOne, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemOne {
            attrs: self.attrs,
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
