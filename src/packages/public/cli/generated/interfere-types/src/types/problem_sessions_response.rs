pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ProblemSessionsResponse {
    #[serde(rename = "hasMore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<ProblemSessionsResponseAvailability>,
    #[serde(default)]
    pub sessions: Vec<ProblemSessionsResponseSessionsItem>,
}

impl ProblemSessionsResponse {
    pub fn builder() -> ProblemSessionsResponseBuilder {
        <ProblemSessionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemSessionsResponseBuilder {
    has_more: Option<bool>,
    availability: Option<ProblemSessionsResponseAvailability>,
    sessions: Option<Vec<ProblemSessionsResponseSessionsItem>>,
}

impl ProblemSessionsResponseBuilder {
    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn availability(mut self, value: ProblemSessionsResponseAvailability) -> Self {
        self.availability = Some(value);
        self
    }

    pub fn sessions(mut self, value: Vec<ProblemSessionsResponseSessionsItem>) -> Self {
        self.sessions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProblemSessionsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sessions`](ProblemSessionsResponseBuilder::sessions)
    pub fn build(self) -> Result<ProblemSessionsResponse, BuildError> {
        Ok(ProblemSessionsResponse {
            has_more: self.has_more,
            availability: self.availability,
            sessions: self.sessions.ok_or_else(|| BuildError::missing_field("sessions"))?,
        })
    }
}
