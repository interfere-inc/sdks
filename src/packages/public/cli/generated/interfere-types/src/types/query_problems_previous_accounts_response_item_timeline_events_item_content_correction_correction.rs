pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supersedes: Option<String>,
    pub reason: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrectionReason,
    #[serde(default)]
    pub locations: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrectionLocationsItem>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrection {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrectionBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrectionBuilder {
    supersedes: Option<String>,
    reason: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrectionReason>,
    locations: Option<Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrectionLocationsItem>>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrectionBuilder {
    pub fn supersedes(mut self, value: impl Into<String>) -> Self {
        self.supersedes = Some(value.into());
        self
    }

    pub fn reason(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrectionReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn locations(mut self, value: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrectionLocationsItem>) -> Self {
        self.locations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reason`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrectionBuilder::reason)
    /// - [`locations`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrectionBuilder::locations)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrection, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrection {
            supersedes: self.supersedes,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
            locations: self.locations.ok_or_else(|| BuildError::missing_field("locations"))?,
        })
    }
}
