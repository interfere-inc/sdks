pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionIntegrationName {
    pub kind: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionIntegrationNameKind,
    #[serde(rename = "integrationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_name: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionIntegrationName {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionIntegrationNameBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionIntegrationNameBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionIntegrationNameBuilder {
    kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionIntegrationNameKind>,
    integration_name: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionIntegrationNameBuilder {
    pub fn kind(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionIntegrationNameKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn integration_name(mut self, value: impl Into<String>) -> Self {
        self.integration_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionIntegrationName`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionIntegrationNameBuilder::kind)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionIntegrationName, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionIntegrationName {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            integration_name: self.integration_name,
        })
    }
}
