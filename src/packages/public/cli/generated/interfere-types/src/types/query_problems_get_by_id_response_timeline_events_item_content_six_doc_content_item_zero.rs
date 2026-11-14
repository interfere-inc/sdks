pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemZero {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<Option<serde_json::Value>>>,
    pub r#type: QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemZeroType,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemZero {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemZeroBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemZeroBuilder {
    content: Option<Vec<Option<serde_json::Value>>>,
    r#type: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemZeroType>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemZeroBuilder {
    pub fn content(mut self, value: Vec<Option<serde_json::Value>>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemZeroBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemZero, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemZero {
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
