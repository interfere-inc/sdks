pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOne {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOneAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOneContentItem>>,
    pub r#type: QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOneType,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOne {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOneBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOneBuilder {
    attrs: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOneAttrs>,
    content: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOneContentItem>>,
    r#type: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOneType>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOneBuilder {
    pub fn attrs(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOneAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn content(mut self, value: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOneContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOneBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOne, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOne {
            attrs: self.attrs,
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
