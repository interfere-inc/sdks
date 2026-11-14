pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProblemSourceResponseBuildsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(default)]
    pub version: String,
    #[serde(rename = "commitSha")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_sha: Option<String>,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "firstSeenAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen_at: Option<i64>,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "lastSeenAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<i64>,
}

impl ProblemSourceResponseBuildsItem {
    pub fn builder() -> ProblemSourceResponseBuildsItemBuilder {
        <ProblemSourceResponseBuildsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemSourceResponseBuildsItemBuilder {
    environment: Option<String>,
    service_name: Option<String>,
    version: Option<String>,
    commit_sha: Option<String>,
    first_seen_at: Option<i64>,
    last_seen_at: Option<i64>,
}

impl ProblemSourceResponseBuildsItemBuilder {
    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn service_name(mut self, value: impl Into<String>) -> Self {
        self.service_name = Some(value.into());
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn commit_sha(mut self, value: impl Into<String>) -> Self {
        self.commit_sha = Some(value.into());
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

    /// Consumes the builder and constructs a [`ProblemSourceResponseBuildsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](ProblemSourceResponseBuildsItemBuilder::version)
    pub fn build(self) -> Result<ProblemSourceResponseBuildsItem, BuildError> {
        Ok(ProblemSourceResponseBuildsItem {
            environment: self.environment,
            service_name: self.service_name,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            commit_sha: self.commit_sha,
            first_seen_at: self.first_seen_at,
            last_seen_at: self.last_seen_at,
        })
    }
}
