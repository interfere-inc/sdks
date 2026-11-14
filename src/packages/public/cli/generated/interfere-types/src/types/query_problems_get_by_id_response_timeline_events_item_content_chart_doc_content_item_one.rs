pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOne {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOneAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOneContentItem>>,
    pub r#type: QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOneType,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOne {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOneBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOneBuilder {
    attrs: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOneAttrs>,
    content: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOneContentItem>>,
    r#type: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOneType>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOneBuilder {
    pub fn attrs(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOneAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn content(mut self, value: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOneContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOneBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOne, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOne {
            attrs: self.attrs,
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
