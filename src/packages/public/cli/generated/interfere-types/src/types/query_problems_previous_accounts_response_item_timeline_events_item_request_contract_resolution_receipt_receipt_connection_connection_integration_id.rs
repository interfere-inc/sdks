pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionIntegrationId {
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<serde_json::Value>,
    #[serde(rename = "datasourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasource_id: Option<serde_json::Value>,
    #[serde(rename = "connectionRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_revision: Option<serde_json::Value>,
    #[serde(rename = "verifiedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<serde_json::Value>,
    #[serde(rename = "integrationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<serde_json::Value>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionIntegrationId {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionIntegrationIdBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionIntegrationIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionIntegrationIdBuilder {
    target_id: Option<serde_json::Value>,
    datasource_id: Option<serde_json::Value>,
    connection_revision: Option<serde_json::Value>,
    verified_at: Option<serde_json::Value>,
    integration_id: Option<serde_json::Value>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionIntegrationIdBuilder {
    pub fn target_id(mut self, value: serde_json::Value) -> Self {
        self.target_id = Some(value);
        self
    }

    pub fn datasource_id(mut self, value: serde_json::Value) -> Self {
        self.datasource_id = Some(value);
        self
    }

    pub fn connection_revision(mut self, value: serde_json::Value) -> Self {
        self.connection_revision = Some(value);
        self
    }

    pub fn verified_at(mut self, value: serde_json::Value) -> Self {
        self.verified_at = Some(value);
        self
    }

    pub fn integration_id(mut self, value: serde_json::Value) -> Self {
        self.integration_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionIntegrationId`].
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionIntegrationId, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionIntegrationId {
            target_id: self.target_id,
            datasource_id: self.datasource_id,
            connection_revision: self.connection_revision,
            verified_at: self.verified_at,
            integration_id: self.integration_id,
        })
    }
}
