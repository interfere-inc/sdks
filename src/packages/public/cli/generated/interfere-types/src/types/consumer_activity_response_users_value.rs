pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConsumerActivityResponseUsersValue {
    #[serde(rename = "problemCount")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub problem_count: f64,
    #[serde(rename = "sessionCount")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub session_count: f64,
    #[serde(default)]
    pub sparkline: Vec<f64>,
}

impl ConsumerActivityResponseUsersValue {
    pub fn builder() -> ConsumerActivityResponseUsersValueBuilder {
        <ConsumerActivityResponseUsersValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConsumerActivityResponseUsersValueBuilder {
    problem_count: Option<f64>,
    session_count: Option<f64>,
    sparkline: Option<Vec<f64>>,
}

impl ConsumerActivityResponseUsersValueBuilder {
    pub fn problem_count(mut self, value: f64) -> Self {
        self.problem_count = Some(value);
        self
    }

    pub fn session_count(mut self, value: f64) -> Self {
        self.session_count = Some(value);
        self
    }

    pub fn sparkline(mut self, value: Vec<f64>) -> Self {
        self.sparkline = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConsumerActivityResponseUsersValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`problem_count`](ConsumerActivityResponseUsersValueBuilder::problem_count)
    /// - [`session_count`](ConsumerActivityResponseUsersValueBuilder::session_count)
    /// - [`sparkline`](ConsumerActivityResponseUsersValueBuilder::sparkline)
    pub fn build(self) -> Result<ConsumerActivityResponseUsersValue, BuildError> {
        Ok(ConsumerActivityResponseUsersValue {
            problem_count: self.problem_count.ok_or_else(|| BuildError::missing_field("problem_count"))?,
            session_count: self.session_count.ok_or_else(|| BuildError::missing_field("session_count"))?,
            sparkline: self.sparkline.ok_or_else(|| BuildError::missing_field("sparkline"))?,
        })
    }
}
