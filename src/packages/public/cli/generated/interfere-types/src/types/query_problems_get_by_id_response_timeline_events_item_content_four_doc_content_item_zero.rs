pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemZero {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<Option<serde_json::Value>>>,
    pub r#type: QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemZeroType,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemZero {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemZeroBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemZeroBuilder {
    content: Option<Vec<Option<serde_json::Value>>>,
    r#type: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemZeroType>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemZeroBuilder {
    pub fn content(mut self, value: Vec<Option<serde_json::Value>>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemZeroBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemZero, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemZero {
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
