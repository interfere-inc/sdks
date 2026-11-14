pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCategory {
    pub category: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCategoryCategory>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCategory {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCategoryBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCategoryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCategoryBuilder {
    category: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCategoryCategory>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCategoryBuilder {
    pub fn category(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCategoryCategory) -> Self {
        self.category = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCategory`].
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCategory, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCategory {
            category: self.category,
        })
    }
}
