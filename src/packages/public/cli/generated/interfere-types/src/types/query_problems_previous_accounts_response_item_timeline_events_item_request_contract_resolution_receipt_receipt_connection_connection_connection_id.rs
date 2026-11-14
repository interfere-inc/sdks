pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId {
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<serde_json::Value>,
    #[serde(rename = "connectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<serde_json::Value>,
    #[serde(rename = "datasourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasource_id: Option<serde_json::Value>,
    #[serde(rename = "connectionRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_revision: Option<serde_json::Value>,
    #[serde(rename = "verifiedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<serde_json::Value>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionConnectionIdBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionConnectionIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionConnectionIdBuilder {
    target_id: Option<serde_json::Value>,
    connection_id: Option<serde_json::Value>,
    datasource_id: Option<serde_json::Value>,
    connection_revision: Option<serde_json::Value>,
    verified_at: Option<serde_json::Value>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionConnectionIdBuilder {
    pub fn target_id(mut self, value: serde_json::Value) -> Self {
        self.target_id = Some(value);
        self
    }

    pub fn connection_id(mut self, value: serde_json::Value) -> Self {
        self.connection_id = Some(value);
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

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId`].
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId {
            target_id: self.target_id,
            connection_id: self.connection_id,
            datasource_id: self.datasource_id,
            connection_revision: self.connection_revision,
            verified_at: self.verified_at,
        })
    }
}
