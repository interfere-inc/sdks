pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCosineScore {
    #[serde(rename = "cosineScore")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub cosine_score: f64,
    #[serde(rename = "findingId")]
    #[serde(default)]
    pub finding_id: String,
    #[serde(rename = "fingerprintMatchExact")]
    #[serde(default)]
    pub fingerprint_match_exact: bool,
    #[serde(rename = "regressionProbability")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub regression_probability: f64,
    #[serde(rename = "resolvedProblemId")]
    #[serde(default)]
    pub resolved_problem_id: String,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCosineScore {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCosineScoreBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCosineScoreBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCosineScoreBuilder {
    cosine_score: Option<f64>,
    finding_id: Option<String>,
    fingerprint_match_exact: Option<bool>,
    regression_probability: Option<f64>,
    resolved_problem_id: Option<String>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCosineScoreBuilder {
    pub fn cosine_score(mut self, value: f64) -> Self {
        self.cosine_score = Some(value);
        self
    }

    pub fn finding_id(mut self, value: impl Into<String>) -> Self {
        self.finding_id = Some(value.into());
        self
    }

    pub fn fingerprint_match_exact(mut self, value: bool) -> Self {
        self.fingerprint_match_exact = Some(value);
        self
    }

    pub fn regression_probability(mut self, value: f64) -> Self {
        self.regression_probability = Some(value);
        self
    }

    pub fn resolved_problem_id(mut self, value: impl Into<String>) -> Self {
        self.resolved_problem_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCosineScore`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cosine_score`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCosineScoreBuilder::cosine_score)
    /// - [`finding_id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCosineScoreBuilder::finding_id)
    /// - [`fingerprint_match_exact`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCosineScoreBuilder::fingerprint_match_exact)
    /// - [`regression_probability`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCosineScoreBuilder::regression_probability)
    /// - [`resolved_problem_id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCosineScoreBuilder::resolved_problem_id)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCosineScore, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCosineScore {
            cosine_score: self.cosine_score.ok_or_else(|| BuildError::missing_field("cosine_score"))?,
            finding_id: self.finding_id.ok_or_else(|| BuildError::missing_field("finding_id"))?,
            fingerprint_match_exact: self.fingerprint_match_exact.ok_or_else(|| BuildError::missing_field("fingerprint_match_exact"))?,
            regression_probability: self.regression_probability.ok_or_else(|| BuildError::missing_field("regression_probability"))?,
            resolved_problem_id: self.resolved_problem_id.ok_or_else(|| BuildError::missing_field("resolved_problem_id"))?,
        })
    }
}
