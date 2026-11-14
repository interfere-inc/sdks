pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTitle {
    pub title: String,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTitle {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTitleBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTitleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTitleBuilder {
    title: Option<String>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTitleBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTitle`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTitleBuilder::title)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTitle, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTitle {
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
