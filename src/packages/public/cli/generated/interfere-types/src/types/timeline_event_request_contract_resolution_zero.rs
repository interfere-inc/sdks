pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventRequestContractResolutionZero {
    pub status: TimelineEventRequestContractResolutionZeroStatus,
}

impl TimelineEventRequestContractResolutionZero {
    pub fn builder() -> TimelineEventRequestContractResolutionZeroBuilder {
        <TimelineEventRequestContractResolutionZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventRequestContractResolutionZeroBuilder {
    status: Option<TimelineEventRequestContractResolutionZeroStatus>,
}

impl TimelineEventRequestContractResolutionZeroBuilder {
    pub fn status(mut self, value: TimelineEventRequestContractResolutionZeroStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventRequestContractResolutionZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](TimelineEventRequestContractResolutionZeroBuilder::status)
    pub fn build(self) -> Result<TimelineEventRequestContractResolutionZero, BuildError> {
        Ok(TimelineEventRequestContractResolutionZero {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
