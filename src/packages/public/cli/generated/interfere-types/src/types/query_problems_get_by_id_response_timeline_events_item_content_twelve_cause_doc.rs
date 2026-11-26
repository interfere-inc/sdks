pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDoc {
    #[serde(default)]
    pub content: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItem>,
    pub r#type: QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocType,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDoc {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocBuilder {
    content: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItem>>,
    r#type: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocType>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocBuilder {
    pub fn content(mut self, value: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDoc`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocBuilder::content)
    /// - [`r#type`](QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDoc, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDoc {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
