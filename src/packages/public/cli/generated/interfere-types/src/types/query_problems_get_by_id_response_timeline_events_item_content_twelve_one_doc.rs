pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDoc {
    #[serde(default)]
    pub content: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItem>,
    pub r#type: QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocType,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDoc {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocBuilder {
    content: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItem>>,
    r#type: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocType>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocBuilder {
    pub fn content(mut self, value: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDoc`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocBuilder::content)
    /// - [`r#type`](QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDoc, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDoc {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
