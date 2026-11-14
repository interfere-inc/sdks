pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentRelease {
    pub release: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentReleaseRelease>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentRelease {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentReleaseBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentReleaseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentReleaseBuilder {
    release: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentReleaseRelease>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentReleaseBuilder {
    pub fn release(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentReleaseRelease) -> Self {
        self.release = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentRelease`].
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentRelease, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentRelease {
            release: self.release,
        })
    }
}
