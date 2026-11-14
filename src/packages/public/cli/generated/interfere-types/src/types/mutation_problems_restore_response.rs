pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationProblemsRestoreResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationProblemsRestoreResponse {
    pub fn builder() -> MutationProblemsRestoreResponseBuilder {
        <MutationProblemsRestoreResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationProblemsRestoreResponseBuilder {
    success: Option<bool>,
}

impl MutationProblemsRestoreResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationProblemsRestoreResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationProblemsRestoreResponseBuilder::success)
    pub fn build(self) -> Result<MutationProblemsRestoreResponse, BuildError> {
        Ok(MutationProblemsRestoreResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
