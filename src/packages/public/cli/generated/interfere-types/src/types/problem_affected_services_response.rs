pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemAffectedServicesResponse {
    pub availability: ProblemAffectedServicesResponseAvailability,
    #[serde(rename = "sampledTraces")]
    #[serde(default)]
    pub sampled_traces: i64,
    #[serde(default)]
    pub services: Vec<ProblemAffectedServicesResponseServicesItem>,
}

impl ProblemAffectedServicesResponse {
    pub fn builder() -> ProblemAffectedServicesResponseBuilder {
        <ProblemAffectedServicesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemAffectedServicesResponseBuilder {
    availability: Option<ProblemAffectedServicesResponseAvailability>,
    sampled_traces: Option<i64>,
    services: Option<Vec<ProblemAffectedServicesResponseServicesItem>>,
}

impl ProblemAffectedServicesResponseBuilder {
    pub fn availability(mut self, value: ProblemAffectedServicesResponseAvailability) -> Self {
        self.availability = Some(value);
        self
    }

    pub fn sampled_traces(mut self, value: i64) -> Self {
        self.sampled_traces = Some(value);
        self
    }

    pub fn services(mut self, value: Vec<ProblemAffectedServicesResponseServicesItem>) -> Self {
        self.services = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProblemAffectedServicesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`availability`](ProblemAffectedServicesResponseBuilder::availability)
    /// - [`sampled_traces`](ProblemAffectedServicesResponseBuilder::sampled_traces)
    /// - [`services`](ProblemAffectedServicesResponseBuilder::services)
    pub fn build(self) -> Result<ProblemAffectedServicesResponse, BuildError> {
        Ok(ProblemAffectedServicesResponse {
            availability: self.availability.ok_or_else(|| BuildError::missing_field("availability"))?,
            sampled_traces: self.sampled_traces.ok_or_else(|| BuildError::missing_field("sampled_traces"))?,
            services: self.services.ok_or_else(|| BuildError::missing_field("services"))?,
        })
    }
}
