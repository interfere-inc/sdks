pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentTwelveCauseCorrection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supersedes: Option<String>,
    pub reason: TimelineEventContentTwelveCauseCorrectionReason,
    #[serde(default)]
    pub locations: Vec<TimelineEventContentTwelveCauseCorrectionLocationsItem>,
}

impl TimelineEventContentTwelveCauseCorrection {
    pub fn builder() -> TimelineEventContentTwelveCauseCorrectionBuilder {
        <TimelineEventContentTwelveCauseCorrectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentTwelveCauseCorrectionBuilder {
    supersedes: Option<String>,
    reason: Option<TimelineEventContentTwelveCauseCorrectionReason>,
    locations: Option<Vec<TimelineEventContentTwelveCauseCorrectionLocationsItem>>,
}

impl TimelineEventContentTwelveCauseCorrectionBuilder {
    pub fn supersedes(mut self, value: impl Into<String>) -> Self {
        self.supersedes = Some(value.into());
        self
    }

    pub fn reason(mut self, value: TimelineEventContentTwelveCauseCorrectionReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn locations(mut self, value: Vec<TimelineEventContentTwelveCauseCorrectionLocationsItem>) -> Self {
        self.locations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentTwelveCauseCorrection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reason`](TimelineEventContentTwelveCauseCorrectionBuilder::reason)
    /// - [`locations`](TimelineEventContentTwelveCauseCorrectionBuilder::locations)
    pub fn build(self) -> Result<TimelineEventContentTwelveCauseCorrection, BuildError> {
        Ok(TimelineEventContentTwelveCauseCorrection {
            supersedes: self.supersedes,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
            locations: self.locations.ok_or_else(|| BuildError::missing_field("locations"))?,
        })
    }
}
