pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractAskedBy {
    #[serde(rename = "runId")]
    #[serde(default)]
    pub run_id: String,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractAskedBy {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemRequestContractAskedByBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemRequestContractAskedByBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractAskedByBuilder {
    run_id: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractAskedByBuilder {
    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemRequestContractAskedBy`].
    /// This method will fail if any of the following fields are not set:
    /// - [`run_id`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractAskedByBuilder::run_id)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractAskedBy, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractAskedBy {
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
        })
    }
}
