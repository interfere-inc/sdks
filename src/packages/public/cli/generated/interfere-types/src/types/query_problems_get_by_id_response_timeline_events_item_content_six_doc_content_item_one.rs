pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOne {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOneAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOneContentItem>>,
    pub r#type: QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOneType,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOne {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOneBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOneBuilder {
    attrs: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOneAttrs>,
    content: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOneContentItem>>,
    r#type: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOneType>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOneBuilder {
    pub fn attrs(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOneAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn content(mut self, value: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOneContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOneBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOne, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOne {
            attrs: self.attrs,
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
