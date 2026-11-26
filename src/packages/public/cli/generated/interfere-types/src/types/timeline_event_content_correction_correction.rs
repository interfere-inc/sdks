pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentCorrectionCorrection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supersedes: Option<String>,
    pub reason: TimelineEventContentCorrectionCorrectionReason,
    #[serde(default)]
    pub locations: Vec<TimelineEventContentCorrectionCorrectionLocationsItem>,
}

impl TimelineEventContentCorrectionCorrection {
    pub fn builder() -> TimelineEventContentCorrectionCorrectionBuilder {
        <TimelineEventContentCorrectionCorrectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentCorrectionCorrectionBuilder {
    supersedes: Option<String>,
    reason: Option<TimelineEventContentCorrectionCorrectionReason>,
    locations: Option<Vec<TimelineEventContentCorrectionCorrectionLocationsItem>>,
}

impl TimelineEventContentCorrectionCorrectionBuilder {
    pub fn supersedes(mut self, value: impl Into<String>) -> Self {
        self.supersedes = Some(value.into());
        self
    }

    pub fn reason(mut self, value: TimelineEventContentCorrectionCorrectionReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn locations(mut self, value: Vec<TimelineEventContentCorrectionCorrectionLocationsItem>) -> Self {
        self.locations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentCorrectionCorrection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reason`](TimelineEventContentCorrectionCorrectionBuilder::reason)
    /// - [`locations`](TimelineEventContentCorrectionCorrectionBuilder::locations)
    pub fn build(self) -> Result<TimelineEventContentCorrectionCorrection, BuildError> {
        Ok(TimelineEventContentCorrectionCorrection {
            supersedes: self.supersedes,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
            locations: self.locations.ok_or_else(|| BuildError::missing_field("locations"))?,
        })
    }
}
