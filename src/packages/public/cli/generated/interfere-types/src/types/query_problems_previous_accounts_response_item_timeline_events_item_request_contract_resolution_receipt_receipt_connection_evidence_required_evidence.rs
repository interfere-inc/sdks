pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability: Option<serde_json::Value>,
    #[serde(rename = "incidentScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident_scope: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency: Option<serde_json::Value>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidence {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceBuilder {
    kind: Option<serde_json::Value>,
    provider: Option<serde_json::Value>,
    capability: Option<serde_json::Value>,
    incident_scope: Option<serde_json::Value>,
    dependency: Option<serde_json::Value>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceBuilder {
    pub fn kind(mut self, value: serde_json::Value) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn provider(mut self, value: serde_json::Value) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn capability(mut self, value: serde_json::Value) -> Self {
        self.capability = Some(value);
        self
    }

    pub fn incident_scope(mut self, value: serde_json::Value) -> Self {
        self.incident_scope = Some(value);
        self
    }

    pub fn dependency(mut self, value: serde_json::Value) -> Self {
        self.dependency = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidence`].
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidence, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidence {
            kind: self.kind,
            provider: self.provider,
            capability: self.capability,
            incident_scope: self.incident_scope,
            dependency: self.dependency,
        })
    }
}
