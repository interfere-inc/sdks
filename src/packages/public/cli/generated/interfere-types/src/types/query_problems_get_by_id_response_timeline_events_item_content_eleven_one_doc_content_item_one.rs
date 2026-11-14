pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemOne {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemOneAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<Option<serde_json::Value>>>,
    pub r#type: QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemOneType,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemOne {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemOneBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemOneBuilder {
    attrs: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemOneAttrs>,
    content: Option<Vec<Option<serde_json::Value>>>,
    r#type: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemOneType>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemOneBuilder {
    pub fn attrs(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemOneAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn content(mut self, value: Vec<Option<serde_json::Value>>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemOneBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemOne, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemOne {
            attrs: self.attrs,
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
