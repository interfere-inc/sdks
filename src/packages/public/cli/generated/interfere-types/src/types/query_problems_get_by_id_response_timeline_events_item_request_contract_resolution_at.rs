pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionAt {
    pub status: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionAtStatus,
    #[serde(default)]
    pub at: i64,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionAt {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionAtBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionAtBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionAtBuilder {
    status: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionAtStatus>,
    at: Option<i64>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionAtBuilder {
    pub fn status(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionAtStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn at(mut self, value: i64) -> Self {
        self.at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionAt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionAtBuilder::status)
    /// - [`at`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionAtBuilder::at)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionAt, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionAt {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            at: self.at.ok_or_else(|| BuildError::missing_field("at"))?,
        })
    }
}
