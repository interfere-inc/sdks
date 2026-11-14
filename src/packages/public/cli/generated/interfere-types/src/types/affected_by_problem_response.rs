pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AffectedByProblemResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage: Option<AffectedByProblemResponseCoverage>,
    #[serde(rename = "observedIdentityCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_identity_count: Option<i64>,
    #[serde(rename = "unavailableIdentityCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable_identity_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<AffectedByProblemResponseAvailability>,
    #[serde(rename = "anonymousDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous_device_count: Option<i64>,
    #[serde(default)]
    pub fingerprint: String,
    #[serde(default)]
    pub users: Vec<AffectedByProblemResponseUsersItem>,
}

impl AffectedByProblemResponse {
    pub fn builder() -> AffectedByProblemResponseBuilder {
        <AffectedByProblemResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AffectedByProblemResponseBuilder {
    coverage: Option<AffectedByProblemResponseCoverage>,
    observed_identity_count: Option<i64>,
    unavailable_identity_count: Option<i64>,
    availability: Option<AffectedByProblemResponseAvailability>,
    anonymous_device_count: Option<i64>,
    fingerprint: Option<String>,
    users: Option<Vec<AffectedByProblemResponseUsersItem>>,
}

impl AffectedByProblemResponseBuilder {
    pub fn coverage(mut self, value: AffectedByProblemResponseCoverage) -> Self {
        self.coverage = Some(value);
        self
    }

    pub fn observed_identity_count(mut self, value: i64) -> Self {
        self.observed_identity_count = Some(value);
        self
    }

    pub fn unavailable_identity_count(mut self, value: i64) -> Self {
        self.unavailable_identity_count = Some(value);
        self
    }

    pub fn availability(mut self, value: AffectedByProblemResponseAvailability) -> Self {
        self.availability = Some(value);
        self
    }

    pub fn anonymous_device_count(mut self, value: i64) -> Self {
        self.anonymous_device_count = Some(value);
        self
    }

    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.fingerprint = Some(value.into());
        self
    }

    pub fn users(mut self, value: Vec<AffectedByProblemResponseUsersItem>) -> Self {
        self.users = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AffectedByProblemResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fingerprint`](AffectedByProblemResponseBuilder::fingerprint)
    /// - [`users`](AffectedByProblemResponseBuilder::users)
    pub fn build(self) -> Result<AffectedByProblemResponse, BuildError> {
        Ok(AffectedByProblemResponse {
            coverage: self.coverage,
            observed_identity_count: self.observed_identity_count,
            unavailable_identity_count: self.unavailable_identity_count,
            availability: self.availability,
            anonymous_device_count: self.anonymous_device_count,
            fingerprint: self.fingerprint.ok_or_else(|| BuildError::missing_field("fingerprint"))?,
            users: self.users.ok_or_else(|| BuildError::missing_field("users"))?,
        })
    }
}
