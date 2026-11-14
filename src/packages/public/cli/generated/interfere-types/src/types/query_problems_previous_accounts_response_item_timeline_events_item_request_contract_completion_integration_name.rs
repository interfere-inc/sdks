pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionIntegrationName {
    pub kind: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionIntegrationNameKind,
    #[serde(rename = "integrationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_name: Option<String>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionIntegrationName {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionIntegrationNameBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionIntegrationNameBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionIntegrationNameBuilder {
    kind: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionIntegrationNameKind>,
    integration_name: Option<String>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionIntegrationNameBuilder {
    pub fn kind(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionIntegrationNameKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn integration_name(mut self, value: impl Into<String>) -> Self {
        self.integration_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionIntegrationName`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionIntegrationNameBuilder::kind)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionIntegrationName, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionIntegrationName {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            integration_name: self.integration_name,
        })
    }
}
