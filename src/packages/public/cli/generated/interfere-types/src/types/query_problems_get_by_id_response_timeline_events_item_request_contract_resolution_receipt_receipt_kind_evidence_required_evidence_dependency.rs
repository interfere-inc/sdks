pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependency {
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<serde_json::Value>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependency {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependencyBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependencyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependencyBuilder {
    group_id: Option<serde_json::Value>,
    mode: Option<serde_json::Value>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependencyBuilder {
    pub fn group_id(mut self, value: serde_json::Value) -> Self {
        self.group_id = Some(value);
        self
    }

    pub fn mode(mut self, value: serde_json::Value) -> Self {
        self.mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependency`].
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependency, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependency {
            group_id: self.group_id,
            mode: self.mode,
        })
    }
}
