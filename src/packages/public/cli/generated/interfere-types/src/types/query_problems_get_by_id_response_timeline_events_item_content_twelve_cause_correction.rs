pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseCorrection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supersedes: Option<String>,
    pub reason: QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseCorrectionReason,
    #[serde(default)]
    pub locations: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseCorrectionLocationsItem>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseCorrection {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseCorrectionBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseCorrectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseCorrectionBuilder {
    supersedes: Option<String>,
    reason: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseCorrectionReason>,
    locations: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseCorrectionLocationsItem>>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseCorrectionBuilder {
    pub fn supersedes(mut self, value: impl Into<String>) -> Self {
        self.supersedes = Some(value.into());
        self
    }

    pub fn reason(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseCorrectionReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn locations(mut self, value: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseCorrectionLocationsItem>) -> Self {
        self.locations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseCorrection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reason`](QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseCorrectionBuilder::reason)
    /// - [`locations`](QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseCorrectionBuilder::locations)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseCorrection, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseCorrection {
            supersedes: self.supersedes,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
            locations: self.locations.ok_or_else(|| BuildError::missing_field("locations"))?,
        })
    }
}
