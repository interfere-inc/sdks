pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProblemTimeseriesResponseDataItemEnvironmentsItem {
    #[serde(default)]
    pub environment: String,
    #[serde(default)]
    pub occurrences: i64,
    #[serde(rename = "affectedUsers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_users: Option<i64>,
}

impl ProblemTimeseriesResponseDataItemEnvironmentsItem {
    pub fn builder() -> ProblemTimeseriesResponseDataItemEnvironmentsItemBuilder {
        <ProblemTimeseriesResponseDataItemEnvironmentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemTimeseriesResponseDataItemEnvironmentsItemBuilder {
    environment: Option<String>,
    occurrences: Option<i64>,
    affected_users: Option<i64>,
}

impl ProblemTimeseriesResponseDataItemEnvironmentsItemBuilder {
    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
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

    /// Consumes the builder and constructs a [`ProblemTimeseriesResponseDataItemEnvironmentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`environment`](ProblemTimeseriesResponseDataItemEnvironmentsItemBuilder::environment)
    /// - [`occurrences`](ProblemTimeseriesResponseDataItemEnvironmentsItemBuilder::occurrences)
    pub fn build(self) -> Result<ProblemTimeseriesResponseDataItemEnvironmentsItem, BuildError> {
        Ok(ProblemTimeseriesResponseDataItemEnvironmentsItem {
            environment: self.environment.ok_or_else(|| BuildError::missing_field("environment"))?,
            occurrences: self.occurrences.ok_or_else(|| BuildError::missing_field("occurrences"))?,
            affected_users: self.affected_users,
        })
    }
}
