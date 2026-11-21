pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MarkInRecoveryProblemsRequestArgsFixKind {
    pub kind: MarkInRecoveryProblemsRequestArgsFixKindKind,
    #[serde(rename = "pullRequest")]
    pub pull_request: MarkInRecoveryProblemsRequestArgsFixKindPullRequest,
}

impl MarkInRecoveryProblemsRequestArgsFixKind {
    pub fn builder() -> MarkInRecoveryProblemsRequestArgsFixKindBuilder {
        <MarkInRecoveryProblemsRequestArgsFixKindBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MarkInRecoveryProblemsRequestArgsFixKindBuilder {
    kind: Option<MarkInRecoveryProblemsRequestArgsFixKindKind>,
    pull_request: Option<MarkInRecoveryProblemsRequestArgsFixKindPullRequest>,
}

impl MarkInRecoveryProblemsRequestArgsFixKindBuilder {
    pub fn kind(mut self, value: MarkInRecoveryProblemsRequestArgsFixKindKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn pull_request(mut self, value: MarkInRecoveryProblemsRequestArgsFixKindPullRequest) -> Self {
        self.pull_request = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MarkInRecoveryProblemsRequestArgsFixKind`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](MarkInRecoveryProblemsRequestArgsFixKindBuilder::kind)
    /// - [`pull_request`](MarkInRecoveryProblemsRequestArgsFixKindBuilder::pull_request)
    pub fn build(self) -> Result<MarkInRecoveryProblemsRequestArgsFixKind, BuildError> {
        Ok(MarkInRecoveryProblemsRequestArgsFixKind {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            pull_request: self.pull_request.ok_or_else(|| BuildError::missing_field("pull_request"))?,
        })
    }
}
