pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationEnvironmentsDeleteResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationEnvironmentsDeleteResponse {
    pub fn builder() -> MutationEnvironmentsDeleteResponseBuilder {
        <MutationEnvironmentsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationEnvironmentsDeleteResponseBuilder {
    success: Option<bool>,
}

impl MutationEnvironmentsDeleteResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationEnvironmentsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationEnvironmentsDeleteResponseBuilder::success)
    pub fn build(self) -> Result<MutationEnvironmentsDeleteResponse, BuildError> {
        Ok(MutationEnvironmentsDeleteResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
