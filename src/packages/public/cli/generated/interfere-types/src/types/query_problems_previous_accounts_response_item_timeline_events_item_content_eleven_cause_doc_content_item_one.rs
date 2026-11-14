pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCauseDocContentItemOne {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<serde_json::Value>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCauseDocContentItemOne {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCauseDocContentItemOneBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCauseDocContentItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCauseDocContentItemOneBuilder {
    attrs: Option<serde_json::Value>,
    content: Option<serde_json::Value>,
    r#type: Option<serde_json::Value>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCauseDocContentItemOneBuilder {
    pub fn attrs(mut self, value: serde_json::Value) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn content(mut self, value: serde_json::Value) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: serde_json::Value) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCauseDocContentItemOne`].
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCauseDocContentItemOne, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCauseDocContentItemOne {
            attrs: self.attrs,
            content: self.content,
            r#type: self.r#type,
        })
    }
}
