pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemZero {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<Option<serde_json::Value>>>,
    pub r#type: QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemZeroType,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemZero {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemZeroBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemZeroBuilder {
    content: Option<Vec<Option<serde_json::Value>>>,
    r#type: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemZeroType>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemZeroBuilder {
    pub fn content(mut self, value: Vec<Option<serde_json::Value>>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemZeroBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemZero, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemZero {
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
