pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCauseDocContentItemType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<serde_json::Value>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCauseDocContentItemType {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCauseDocContentItemTypeBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCauseDocContentItemTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCauseDocContentItemTypeBuilder {
    attrs: Option<serde_json::Value>,
    r#type: Option<serde_json::Value>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCauseDocContentItemTypeBuilder {
    pub fn attrs(mut self, value: serde_json::Value) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: serde_json::Value) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCauseDocContentItemType`].
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCauseDocContentItemType, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCauseDocContentItemType {
            attrs: self.attrs,
            r#type: self.r#type,
        })
    }
}
