pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MarkInRecoveryProblemsRequestArgsFixRelease {
    pub kind: MarkInRecoveryProblemsRequestArgsFixReleaseKind,
    #[serde(default)]
    pub release: String,
}

impl MarkInRecoveryProblemsRequestArgsFixRelease {
    pub fn builder() -> MarkInRecoveryProblemsRequestArgsFixReleaseBuilder {
        <MarkInRecoveryProblemsRequestArgsFixReleaseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MarkInRecoveryProblemsRequestArgsFixReleaseBuilder {
    kind: Option<MarkInRecoveryProblemsRequestArgsFixReleaseKind>,
    release: Option<String>,
}

impl MarkInRecoveryProblemsRequestArgsFixReleaseBuilder {
    pub fn kind(mut self, value: MarkInRecoveryProblemsRequestArgsFixReleaseKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn release(mut self, value: impl Into<String>) -> Self {
        self.release = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MarkInRecoveryProblemsRequestArgsFixRelease`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](MarkInRecoveryProblemsRequestArgsFixReleaseBuilder::kind)
    /// - [`release`](MarkInRecoveryProblemsRequestArgsFixReleaseBuilder::release)
    pub fn build(self) -> Result<MarkInRecoveryProblemsRequestArgsFixRelease, BuildError> {
        Ok(MarkInRecoveryProblemsRequestArgsFixRelease {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            release: self.release.ok_or_else(|| BuildError::missing_field("release"))?,
        })
    }
}
