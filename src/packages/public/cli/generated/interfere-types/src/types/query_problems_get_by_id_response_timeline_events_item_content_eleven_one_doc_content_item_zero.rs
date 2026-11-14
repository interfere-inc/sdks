pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemZero {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<Option<serde_json::Value>>>,
    pub r#type: QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemZeroType,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemZero {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemZeroBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemZeroBuilder {
    content: Option<Vec<Option<serde_json::Value>>>,
    r#type: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemZeroType>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemZeroBuilder {
    pub fn content(mut self, value: Vec<Option<serde_json::Value>>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemZeroBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemZero, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemZero {
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
