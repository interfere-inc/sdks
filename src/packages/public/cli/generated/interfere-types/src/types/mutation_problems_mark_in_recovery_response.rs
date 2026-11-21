pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationProblemsMarkInRecoveryResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationProblemsMarkInRecoveryResponse {
    pub fn builder() -> MutationProblemsMarkInRecoveryResponseBuilder {
        <MutationProblemsMarkInRecoveryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationProblemsMarkInRecoveryResponseBuilder {
    success: Option<bool>,
}

impl MutationProblemsMarkInRecoveryResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationProblemsMarkInRecoveryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationProblemsMarkInRecoveryResponseBuilder::success)
    pub fn build(self) -> Result<MutationProblemsMarkInRecoveryResponse, BuildError> {
        Ok(MutationProblemsMarkInRecoveryResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
