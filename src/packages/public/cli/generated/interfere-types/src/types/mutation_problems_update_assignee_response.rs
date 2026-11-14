pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationProblemsUpdateAssigneeResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationProblemsUpdateAssigneeResponse {
    pub fn builder() -> MutationProblemsUpdateAssigneeResponseBuilder {
        <MutationProblemsUpdateAssigneeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationProblemsUpdateAssigneeResponseBuilder {
    success: Option<bool>,
}

impl MutationProblemsUpdateAssigneeResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationProblemsUpdateAssigneeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationProblemsUpdateAssigneeResponseBuilder::success)
    pub fn build(self) -> Result<MutationProblemsUpdateAssigneeResponse, BuildError> {
        Ok(MutationProblemsUpdateAssigneeResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
