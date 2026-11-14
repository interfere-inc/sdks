pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemSourceResponse {
    pub ownership: ProblemSourceResponseOwnership,
    /// The repository linked to the failing code; null when none is linked
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<ProblemSourceResponseRepository>,
    /// Commit of the newest captured occurrence: the service.version a server reported, or the release a browser bundle shipped in
    #[serde(rename = "failingCommit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failing_commit: Option<String>,
    #[serde(rename = "failingCommitUnavailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failing_commit_unavailable: Option<ProblemSourceResponseFailingCommitUnavailable>,
    /// Builds after the failure, oldest first. For a server service: each service.version seen in its telemetry, firstSeenAt being when it began serving; overlapping builds served at once. For a browser app: later releases, firstSeenAt being the deployment time.
    #[serde(default)]
    pub builds: Vec<ProblemSourceResponseBuildsItem>,
}

impl ProblemSourceResponse {
    pub fn builder() -> ProblemSourceResponseBuilder {
        <ProblemSourceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemSourceResponseBuilder {
    ownership: Option<ProblemSourceResponseOwnership>,
    repository: Option<ProblemSourceResponseRepository>,
    failing_commit: Option<String>,
    failing_commit_unavailable: Option<ProblemSourceResponseFailingCommitUnavailable>,
    builds: Option<Vec<ProblemSourceResponseBuildsItem>>,
}

impl ProblemSourceResponseBuilder {
    pub fn ownership(mut self, value: ProblemSourceResponseOwnership) -> Self {
        self.ownership = Some(value);
        self
    }

    pub fn repository(mut self, value: ProblemSourceResponseRepository) -> Self {
        self.repository = Some(value);
        self
    }

    pub fn failing_commit(mut self, value: impl Into<String>) -> Self {
        self.failing_commit = Some(value.into());
        self
    }

    pub fn failing_commit_unavailable(mut self, value: ProblemSourceResponseFailingCommitUnavailable) -> Self {
        self.failing_commit_unavailable = Some(value);
        self
    }

    pub fn builds(mut self, value: Vec<ProblemSourceResponseBuildsItem>) -> Self {
        self.builds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProblemSourceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ownership`](ProblemSourceResponseBuilder::ownership)
    /// - [`builds`](ProblemSourceResponseBuilder::builds)
    pub fn build(self) -> Result<ProblemSourceResponse, BuildError> {
        Ok(ProblemSourceResponse {
            ownership: self.ownership.ok_or_else(|| BuildError::missing_field("ownership"))?,
            repository: self.repository,
            failing_commit: self.failing_commit,
            failing_commit_unavailable: self.failing_commit_unavailable,
            builds: self.builds.ok_or_else(|| BuildError::missing_field("builds"))?,
        })
    }
}
