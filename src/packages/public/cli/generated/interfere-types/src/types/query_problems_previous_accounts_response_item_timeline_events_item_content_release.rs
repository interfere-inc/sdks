pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentRelease {
    pub release: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentReleaseRelease>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentRelease {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentReleaseBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentReleaseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentReleaseBuilder {
    release: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentReleaseRelease>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentReleaseBuilder {
    pub fn release(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentReleaseRelease) -> Self {
        self.release = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentRelease`].
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentRelease, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentRelease {
            release: self.release,
        })
    }
}
