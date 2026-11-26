pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supersedes: Option<String>,
    pub reason: QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrectionReason,
    #[serde(default)]
    pub locations: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrectionLocationsItem>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrection {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrectionBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrectionBuilder {
    supersedes: Option<String>,
    reason: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrectionReason>,
    locations: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrectionLocationsItem>>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrectionBuilder {
    pub fn supersedes(mut self, value: impl Into<String>) -> Self {
        self.supersedes = Some(value.into());
        self
    }

    pub fn reason(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrectionReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn locations(mut self, value: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrectionLocationsItem>) -> Self {
        self.locations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reason`](QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrectionBuilder::reason)
    /// - [`locations`](QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrectionBuilder::locations)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrection, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrection {
            supersedes: self.supersedes,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
            locations: self.locations.ok_or_else(|| BuildError::missing_field("locations"))?,
        })
    }
}
