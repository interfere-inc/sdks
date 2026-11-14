pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentTitle {
    pub title: String,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentTitle {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentTitleBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentTitleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentTitleBuilder {
    title: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentTitleBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentTitle`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](QueryProblemsGetByIdResponseTimelineEventsItemContentTitleBuilder::title)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentTitle, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentTitle {
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
