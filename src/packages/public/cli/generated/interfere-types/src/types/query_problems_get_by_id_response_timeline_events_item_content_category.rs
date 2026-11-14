pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentCategory {
    pub category: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentCategoryCategory>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentCategory {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentCategoryBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentCategoryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentCategoryBuilder {
    category: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentCategoryCategory>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentCategoryBuilder {
    pub fn category(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentCategoryCategory) -> Self {
        self.category = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentCategory`].
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentCategory, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentCategory {
            category: self.category,
        })
    }
}
