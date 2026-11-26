pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractAskedBy {
    #[serde(rename = "runId")]
    #[serde(default)]
    pub run_id: String,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractAskedBy {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractAskedByBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractAskedByBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractAskedByBuilder {
    run_id: Option<String>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractAskedByBuilder {
    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractAskedBy`].
    /// This method will fail if any of the following fields are not set:
    /// - [`run_id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractAskedByBuilder::run_id)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractAskedBy, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractAskedBy {
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
        })
    }
}
