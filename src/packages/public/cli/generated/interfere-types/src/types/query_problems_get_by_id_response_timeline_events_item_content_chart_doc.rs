pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentChartDoc {
    #[serde(default)]
    pub content: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItem>,
    pub r#type: QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocType,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentChartDoc {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocBuilder {
    content: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItem>>,
    r#type: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocType>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocBuilder {
    pub fn content(mut self, value: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentChartDoc`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocBuilder::content)
    /// - [`r#type`](QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentChartDoc, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentChartDoc {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
