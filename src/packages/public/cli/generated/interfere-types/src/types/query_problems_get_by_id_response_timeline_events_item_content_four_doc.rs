pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentFourDoc {
    #[serde(default)]
    pub content: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItem>,
    pub r#type: QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocType,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentFourDoc {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocBuilder {
    content: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItem>>,
    r#type: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocType>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocBuilder {
    pub fn content(mut self, value: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentFourDoc`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocBuilder::content)
    /// - [`r#type`](QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentFourDoc, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentFourDoc {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
