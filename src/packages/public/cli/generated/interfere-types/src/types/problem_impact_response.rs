pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProblemImpactResponse {
    #[serde(rename = "windowDays")]
    #[serde(default)]
    pub window_days: String,
    #[serde(default)]
    pub occurrences: i64,
    /// Recorded identified users and anonymous devices; null for server Problems, which record no people
    #[serde(rename = "affectedUsers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_users: Option<i64>,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "firstSeenAt")]
    #[serde(default)]
    pub first_seen_at: i64,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "lastSeenAt")]
    #[serde(default)]
    pub last_seen_at: i64,
}

impl ProblemImpactResponse {
    pub fn builder() -> ProblemImpactResponseBuilder {
        <ProblemImpactResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemImpactResponseBuilder {
    window_days: Option<String>,
    occurrences: Option<i64>,
    affected_users: Option<i64>,
    first_seen_at: Option<i64>,
    last_seen_at: Option<i64>,
}

impl ProblemImpactResponseBuilder {
    pub fn window_days(mut self, value: impl Into<String>) -> Self {
        self.window_days = Some(value.into());
        self
    }

    pub fn occurrences(mut self, value: i64) -> Self {
        self.occurrences = Some(value);
        self
    }

    pub fn affected_users(mut self, value: i64) -> Self {
        self.affected_users = Some(value);
        self
    }

    pub fn first_seen_at(mut self, value: i64) -> Self {
        self.first_seen_at = Some(value);
        self
    }

    pub fn last_seen_at(mut self, value: i64) -> Self {
        self.last_seen_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProblemImpactResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`window_days`](ProblemImpactResponseBuilder::window_days)
    /// - [`occurrences`](ProblemImpactResponseBuilder::occurrences)
    /// - [`first_seen_at`](ProblemImpactResponseBuilder::first_seen_at)
    /// - [`last_seen_at`](ProblemImpactResponseBuilder::last_seen_at)
    pub fn build(self) -> Result<ProblemImpactResponse, BuildError> {
        Ok(ProblemImpactResponse {
            window_days: self.window_days.ok_or_else(|| BuildError::missing_field("window_days"))?,
            occurrences: self.occurrences.ok_or_else(|| BuildError::missing_field("occurrences"))?,
            affected_users: self.affected_users,
            first_seen_at: self.first_seen_at.ok_or_else(|| BuildError::missing_field("first_seen_at"))?,
            last_seen_at: self.last_seen_at.ok_or_else(|| BuildError::missing_field("last_seen_at"))?,
        })
    }
}
