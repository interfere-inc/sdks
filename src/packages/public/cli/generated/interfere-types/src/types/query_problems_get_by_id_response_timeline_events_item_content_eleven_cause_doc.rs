pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDoc {
    #[serde(default)]
    pub content: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItem>,
    pub r#type: QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocType,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDoc {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocBuilder {
    content: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItem>>,
    r#type: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocType>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocBuilder {
    pub fn content(mut self, value: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDoc`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocBuilder::content)
    /// - [`r#type`](QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDoc, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDoc {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
