pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationProblemsUpdatePriorityResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationProblemsUpdatePriorityResponse {
    pub fn builder() -> MutationProblemsUpdatePriorityResponseBuilder {
        <MutationProblemsUpdatePriorityResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationProblemsUpdatePriorityResponseBuilder {
    success: Option<bool>,
}

impl MutationProblemsUpdatePriorityResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationProblemsUpdatePriorityResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationProblemsUpdatePriorityResponseBuilder::success)
    pub fn build(self) -> Result<MutationProblemsUpdatePriorityResponse, BuildError> {
        Ok(MutationProblemsUpdatePriorityResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
