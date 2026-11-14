pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AffectedByProblemTimeseriesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage: Option<AffectedByProblemTimeseriesResponseCoverage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<AffectedByProblemTimeseriesResponseAvailability>,
    #[serde(default)]
    pub data: Vec<AffectedByProblemTimeseriesResponseDataItem>,
}

impl AffectedByProblemTimeseriesResponse {
    pub fn builder() -> AffectedByProblemTimeseriesResponseBuilder {
        <AffectedByProblemTimeseriesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AffectedByProblemTimeseriesResponseBuilder {
    coverage: Option<AffectedByProblemTimeseriesResponseCoverage>,
    availability: Option<AffectedByProblemTimeseriesResponseAvailability>,
    data: Option<Vec<AffectedByProblemTimeseriesResponseDataItem>>,
}

impl AffectedByProblemTimeseriesResponseBuilder {
    pub fn coverage(mut self, value: AffectedByProblemTimeseriesResponseCoverage) -> Self {
        self.coverage = Some(value);
        self
    }

    pub fn availability(mut self, value: AffectedByProblemTimeseriesResponseAvailability) -> Self {
        self.availability = Some(value);
        self
    }

    pub fn data(mut self, value: Vec<AffectedByProblemTimeseriesResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AffectedByProblemTimeseriesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](AffectedByProblemTimeseriesResponseBuilder::data)
    pub fn build(self) -> Result<AffectedByProblemTimeseriesResponse, BuildError> {
        Ok(AffectedByProblemTimeseriesResponse {
            coverage: self.coverage,
            availability: self.availability,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
