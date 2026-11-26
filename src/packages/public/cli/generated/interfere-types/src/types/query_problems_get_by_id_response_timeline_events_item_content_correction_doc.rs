pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDoc {
    #[serde(default)]
    pub content: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItem>,
    pub r#type: QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocType,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDoc {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocBuilder {
    content: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItem>>,
    r#type: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocType>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocBuilder {
    pub fn content(mut self, value: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDoc`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocBuilder::content)
    /// - [`r#type`](QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDoc, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDoc {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
