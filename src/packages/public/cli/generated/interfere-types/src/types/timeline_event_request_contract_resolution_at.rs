pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventRequestContractResolutionAt {
    pub status: TimelineEventRequestContractResolutionAtStatus,
    #[serde(default)]
    pub at: i64,
}

impl TimelineEventRequestContractResolutionAt {
    pub fn builder() -> TimelineEventRequestContractResolutionAtBuilder {
        <TimelineEventRequestContractResolutionAtBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventRequestContractResolutionAtBuilder {
    status: Option<TimelineEventRequestContractResolutionAtStatus>,
    at: Option<i64>,
}

impl TimelineEventRequestContractResolutionAtBuilder {
    pub fn status(mut self, value: TimelineEventRequestContractResolutionAtStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn at(mut self, value: i64) -> Self {
        self.at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventRequestContractResolutionAt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](TimelineEventRequestContractResolutionAtBuilder::status)
    /// - [`at`](TimelineEventRequestContractResolutionAtBuilder::at)
    pub fn build(self) -> Result<TimelineEventRequestContractResolutionAt, BuildError> {
        Ok(TimelineEventRequestContractResolutionAt {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            at: self.at.ok_or_else(|| BuildError::missing_field("at"))?,
        })
    }
}
