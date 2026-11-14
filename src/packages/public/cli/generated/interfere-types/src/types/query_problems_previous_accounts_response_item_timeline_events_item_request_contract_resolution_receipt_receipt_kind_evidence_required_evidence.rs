pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidence {
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

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidence {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceBuilder {
    kind: Option<serde_json::Value>,
    provider: Option<serde_json::Value>,
    capability: Option<serde_json::Value>,
    incident_scope: Option<serde_json::Value>,
    dependency: Option<serde_json::Value>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceBuilder {
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

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidence`].
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidence, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidence {
            kind: self.kind,
            provider: self.provider,
            capability: self.capability,
            incident_scope: self.incident_scope,
            dependency: self.dependency,
        })
    }
}
