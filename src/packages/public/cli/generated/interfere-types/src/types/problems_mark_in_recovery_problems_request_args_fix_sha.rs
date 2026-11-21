pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MarkInRecoveryProblemsRequestArgsFixSha {
    pub kind: MarkInRecoveryProblemsRequestArgsFixShaKind,
    #[serde(default)]
    pub sha: String,
}

impl MarkInRecoveryProblemsRequestArgsFixSha {
    pub fn builder() -> MarkInRecoveryProblemsRequestArgsFixShaBuilder {
        <MarkInRecoveryProblemsRequestArgsFixShaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MarkInRecoveryProblemsRequestArgsFixShaBuilder {
    kind: Option<MarkInRecoveryProblemsRequestArgsFixShaKind>,
    sha: Option<String>,
}

impl MarkInRecoveryProblemsRequestArgsFixShaBuilder {
    pub fn kind(mut self, value: MarkInRecoveryProblemsRequestArgsFixShaKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn sha(mut self, value: impl Into<String>) -> Self {
        self.sha = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MarkInRecoveryProblemsRequestArgsFixSha`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](MarkInRecoveryProblemsRequestArgsFixShaBuilder::kind)
    /// - [`sha`](MarkInRecoveryProblemsRequestArgsFixShaBuilder::sha)
    pub fn build(self) -> Result<MarkInRecoveryProblemsRequestArgsFixSha, BuildError> {
        Ok(MarkInRecoveryProblemsRequestArgsFixSha {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            sha: self.sha.ok_or_else(|| BuildError::missing_field("sha"))?,
        })
    }
}
