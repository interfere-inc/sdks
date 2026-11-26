pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supersedes: Option<String>,
    pub reason: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionReason,
    #[serde(default)]
    pub locations: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItem>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrection {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionBuilder {
    supersedes: Option<String>,
    reason: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionReason>,
    locations: Option<Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItem>>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionBuilder {
    pub fn supersedes(mut self, value: impl Into<String>) -> Self {
        self.supersedes = Some(value.into());
        self
    }

    pub fn reason(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn locations(mut self, value: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItem>) -> Self {
        self.locations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reason`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionBuilder::reason)
    /// - [`locations`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionBuilder::locations)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrection, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrection {
            supersedes: self.supersedes,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
            locations: self.locations.ok_or_else(|| BuildError::missing_field("locations"))?,
        })
    }
}
