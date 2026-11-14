pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionZero {
    pub status: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionZeroStatus,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionZero {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionZeroBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionZeroBuilder {
    status: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionZeroStatus>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionZeroBuilder {
    pub fn status(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionZeroStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionZeroBuilder::status)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionZero, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionZero {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
