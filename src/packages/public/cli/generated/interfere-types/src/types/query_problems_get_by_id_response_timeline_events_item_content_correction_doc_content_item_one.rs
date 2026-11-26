pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOne {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOneAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOneContentItem>>,
    pub r#type: QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOneType,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOne {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOneBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOneBuilder {
    attrs: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOneAttrs>,
    content: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOneContentItem>>,
    r#type: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOneType>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOneBuilder {
    pub fn attrs(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOneAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn content(mut self, value: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOneContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOneBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOne, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOne {
            attrs: self.attrs,
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
