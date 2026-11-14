pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDoc {
    #[serde(default)]
    pub content: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItem>,
    pub r#type: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocType,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDoc {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocBuilder {
    content: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItem>>,
    r#type: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocType>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocBuilder {
    pub fn content(mut self, value: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDoc`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocBuilder::content)
    /// - [`r#type`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDoc, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDoc {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
