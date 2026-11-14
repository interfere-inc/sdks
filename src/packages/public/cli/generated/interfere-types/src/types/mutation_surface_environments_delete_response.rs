pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationSurfaceEnvironmentsDeleteResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationSurfaceEnvironmentsDeleteResponse {
    pub fn builder() -> MutationSurfaceEnvironmentsDeleteResponseBuilder {
        <MutationSurfaceEnvironmentsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationSurfaceEnvironmentsDeleteResponseBuilder {
    success: Option<bool>,
}

impl MutationSurfaceEnvironmentsDeleteResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationSurfaceEnvironmentsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationSurfaceEnvironmentsDeleteResponseBuilder::success)
    pub fn build(self) -> Result<MutationSurfaceEnvironmentsDeleteResponse, BuildError> {
        Ok(MutationSurfaceEnvironmentsDeleteResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
