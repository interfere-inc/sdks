pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentSixDoc {
    #[serde(default)]
    pub content: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItem>,
    pub r#type: QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocType,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentSixDoc {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocBuilder {
    content: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItem>>,
    r#type: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocType>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocBuilder {
    pub fn content(mut self, value: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentSixDoc`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocBuilder::content)
    /// - [`r#type`](QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentSixDoc, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentSixDoc {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
