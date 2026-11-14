pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationProblemsUpdateCategoryResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationProblemsUpdateCategoryResponse {
    pub fn builder() -> MutationProblemsUpdateCategoryResponseBuilder {
        <MutationProblemsUpdateCategoryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationProblemsUpdateCategoryResponseBuilder {
    success: Option<bool>,
}

impl MutationProblemsUpdateCategoryResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationProblemsUpdateCategoryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationProblemsUpdateCategoryResponseBuilder::success)
    pub fn build(self) -> Result<MutationProblemsUpdateCategoryResponse, BuildError> {
        Ok(MutationProblemsUpdateCategoryResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
