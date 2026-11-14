pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDoc {
    #[serde(default)]
    pub content: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItem>,
    pub r#type: QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocType,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDoc {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocBuilder {
    content: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItem>>,
    r#type: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocType>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocBuilder {
    pub fn content(mut self, value: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDoc`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocBuilder::content)
    /// - [`r#type`](QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDoc, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDoc {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
