pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionTwo {
    #[serde(default)]
    pub at: i64,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub status: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionTwoStatus,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionTwo {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionTwoBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionTwoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionTwoBuilder {
    at: Option<i64>,
    user_id: Option<String>,
    status: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionTwoStatus>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionTwoBuilder {
    pub fn at(mut self, value: i64) -> Self {
        self.at = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionTwoStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionTwo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`at`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionTwoBuilder::at)
    /// - [`status`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionTwoBuilder::status)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionTwo, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionTwo {
            at: self.at.ok_or_else(|| BuildError::missing_field("at"))?,
            user_id: self.user_id,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
