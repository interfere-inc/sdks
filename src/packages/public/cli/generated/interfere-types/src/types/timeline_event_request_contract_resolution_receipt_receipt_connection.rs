pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventRequestContractResolutionReceiptReceiptConnection {
    pub kind: TimelineEventRequestContractResolutionReceiptReceiptConnectionKind,
    pub connection: TimelineEventRequestContractResolutionReceiptReceiptConnectionConnection,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidence>,
}

impl TimelineEventRequestContractResolutionReceiptReceiptConnection {
    pub fn builder() -> TimelineEventRequestContractResolutionReceiptReceiptConnectionBuilder {
        <TimelineEventRequestContractResolutionReceiptReceiptConnectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventRequestContractResolutionReceiptReceiptConnectionBuilder {
    kind: Option<TimelineEventRequestContractResolutionReceiptReceiptConnectionKind>,
    connection: Option<TimelineEventRequestContractResolutionReceiptReceiptConnectionConnection>,
    evidence: Option<TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidence>,
}

impl TimelineEventRequestContractResolutionReceiptReceiptConnectionBuilder {
    pub fn kind(mut self, value: TimelineEventRequestContractResolutionReceiptReceiptConnectionKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn connection(mut self, value: TimelineEventRequestContractResolutionReceiptReceiptConnectionConnection) -> Self {
        self.connection = Some(value);
        self
    }

    pub fn evidence(mut self, value: TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidence) -> Self {
        self.evidence = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventRequestContractResolutionReceiptReceiptConnection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TimelineEventRequestContractResolutionReceiptReceiptConnectionBuilder::kind)
    /// - [`connection`](TimelineEventRequestContractResolutionReceiptReceiptConnectionBuilder::connection)
    pub fn build(self) -> Result<TimelineEventRequestContractResolutionReceiptReceiptConnection, BuildError> {
        Ok(TimelineEventRequestContractResolutionReceiptReceiptConnection {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            connection: self.connection.ok_or_else(|| BuildError::missing_field("connection"))?,
            evidence: self.evidence,
        })
    }
}
