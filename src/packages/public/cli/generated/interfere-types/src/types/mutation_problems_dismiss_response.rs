pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationProblemsDismissResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationProblemsDismissResponse {
    pub fn builder() -> MutationProblemsDismissResponseBuilder {
        <MutationProblemsDismissResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationProblemsDismissResponseBuilder {
    success: Option<bool>,
}

impl MutationProblemsDismissResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationProblemsDismissResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationProblemsDismissResponseBuilder::success)
    pub fn build(self) -> Result<MutationProblemsDismissResponse, BuildError> {
        Ok(MutationProblemsDismissResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
