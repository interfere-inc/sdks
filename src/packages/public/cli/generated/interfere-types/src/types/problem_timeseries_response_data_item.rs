pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProblemTimeseriesResponseDataItem {
    #[serde(default)]
    pub day: NaiveDate,
    #[serde(default)]
    pub occurrences: i64,
    #[serde(rename = "affectedUsers")]
    #[serde(default)]
    pub affected_users: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environments: Option<Vec<ProblemTimeseriesResponseDataItemEnvironmentsItem>>,
}

impl ProblemTimeseriesResponseDataItem {
    pub fn builder() -> ProblemTimeseriesResponseDataItemBuilder {
        <ProblemTimeseriesResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemTimeseriesResponseDataItemBuilder {
    day: Option<NaiveDate>,
    occurrences: Option<i64>,
    affected_users: Option<i64>,
    environments: Option<Vec<ProblemTimeseriesResponseDataItemEnvironmentsItem>>,
}

impl ProblemTimeseriesResponseDataItemBuilder {
    pub fn day(mut self, value: NaiveDate) -> Self {
        self.day = Some(value);
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

    pub fn environments(mut self, value: Vec<ProblemTimeseriesResponseDataItemEnvironmentsItem>) -> Self {
        self.environments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProblemTimeseriesResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`day`](ProblemTimeseriesResponseDataItemBuilder::day)
    /// - [`occurrences`](ProblemTimeseriesResponseDataItemBuilder::occurrences)
    /// - [`affected_users`](ProblemTimeseriesResponseDataItemBuilder::affected_users)
    pub fn build(self) -> Result<ProblemTimeseriesResponseDataItem, BuildError> {
        Ok(ProblemTimeseriesResponseDataItem {
            day: self.day.ok_or_else(|| BuildError::missing_field("day"))?,
            occurrences: self.occurrences.ok_or_else(|| BuildError::missing_field("occurrences"))?,
            affected_users: self.affected_users.ok_or_else(|| BuildError::missing_field("affected_users"))?,
            environments: self.environments,
        })
    }
}
