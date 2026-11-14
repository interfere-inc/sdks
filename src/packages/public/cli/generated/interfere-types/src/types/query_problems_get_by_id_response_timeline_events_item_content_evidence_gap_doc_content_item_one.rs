pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOne {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOneAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOneContentItem>>,
    pub r#type: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOneType,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOne {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOneBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOneBuilder {
    attrs: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOneAttrs>,
    content: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOneContentItem>>,
    r#type: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOneType>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOneBuilder {
    pub fn attrs(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOneAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn content(mut self, value: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOneContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOneBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOne, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOne {
            attrs: self.attrs,
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
