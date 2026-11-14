pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConsumerLookupResponseLastError {
    /// UTC day of the most recent error.
    #[serde(default)]
    pub day: NaiveDate,
    pub problem: ConsumerLookupResponseLastErrorProblem,
    /// Sessions in which the user hit this problem that day.
    #[serde(rename = "sessionIds")]
    #[serde(default)]
    pub session_ids: Vec<String>,
}

impl ConsumerLookupResponseLastError {
    pub fn builder() -> ConsumerLookupResponseLastErrorBuilder {
        <ConsumerLookupResponseLastErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConsumerLookupResponseLastErrorBuilder {
    day: Option<NaiveDate>,
    problem: Option<ConsumerLookupResponseLastErrorProblem>,
    session_ids: Option<Vec<String>>,
}

impl ConsumerLookupResponseLastErrorBuilder {
    pub fn day(mut self, value: NaiveDate) -> Self {
        self.day = Some(value);
        self
    }

    pub fn problem(mut self, value: ConsumerLookupResponseLastErrorProblem) -> Self {
        self.problem = Some(value);
        self
    }

    pub fn session_ids(mut self, value: Vec<String>) -> Self {
        self.session_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConsumerLookupResponseLastError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`day`](ConsumerLookupResponseLastErrorBuilder::day)
    /// - [`problem`](ConsumerLookupResponseLastErrorBuilder::problem)
    /// - [`session_ids`](ConsumerLookupResponseLastErrorBuilder::session_ids)
    pub fn build(self) -> Result<ConsumerLookupResponseLastError, BuildError> {
        Ok(ConsumerLookupResponseLastError {
            day: self.day.ok_or_else(|| BuildError::missing_field("day"))?,
            problem: self.problem.ok_or_else(|| BuildError::missing_field("problem"))?,
            session_ids: self.session_ids.ok_or_else(|| BuildError::missing_field("session_ids"))?,
        })
    }
}
