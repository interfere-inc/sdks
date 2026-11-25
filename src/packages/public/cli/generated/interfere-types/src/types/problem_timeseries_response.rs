pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProblemTimeseriesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage: Option<ProblemTimeseriesResponseCoverage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environments: Option<Vec<String>>,
    #[serde(default)]
    pub data: Vec<ProblemTimeseriesResponseDataItem>,
}

impl ProblemTimeseriesResponse {
    pub fn builder() -> ProblemTimeseriesResponseBuilder {
        <ProblemTimeseriesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemTimeseriesResponseBuilder {
    coverage: Option<ProblemTimeseriesResponseCoverage>,
    environments: Option<Vec<String>>,
    data: Option<Vec<ProblemTimeseriesResponseDataItem>>,
}

impl ProblemTimeseriesResponseBuilder {
    pub fn coverage(mut self, value: ProblemTimeseriesResponseCoverage) -> Self {
        self.coverage = Some(value);
        self
    }

    pub fn environments(mut self, value: Vec<String>) -> Self {
        self.environments = Some(value);
        self
    }

    pub fn data(mut self, value: Vec<ProblemTimeseriesResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProblemTimeseriesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ProblemTimeseriesResponseBuilder::data)
    pub fn build(self) -> Result<ProblemTimeseriesResponse, BuildError> {
        Ok(ProblemTimeseriesResponse {
            coverage: self.coverage,
            environments: self.environments,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
