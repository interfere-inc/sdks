pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventRequestContractResolutionReceiptReceiptZero {
    pub kind: TimelineEventRequestContractResolutionReceiptReceiptZeroKind,
}

impl TimelineEventRequestContractResolutionReceiptReceiptZero {
    pub fn builder() -> TimelineEventRequestContractResolutionReceiptReceiptZeroBuilder {
        <TimelineEventRequestContractResolutionReceiptReceiptZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventRequestContractResolutionReceiptReceiptZeroBuilder {
    kind: Option<TimelineEventRequestContractResolutionReceiptReceiptZeroKind>,
}

impl TimelineEventRequestContractResolutionReceiptReceiptZeroBuilder {
    pub fn kind(mut self, value: TimelineEventRequestContractResolutionReceiptReceiptZeroKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventRequestContractResolutionReceiptReceiptZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TimelineEventRequestContractResolutionReceiptReceiptZeroBuilder::kind)
    pub fn build(self) -> Result<TimelineEventRequestContractResolutionReceiptReceiptZero, BuildError> {
        Ok(TimelineEventRequestContractResolutionReceiptReceiptZero {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}
