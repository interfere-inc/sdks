pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationSurfaceEnvironmentsSetProductionResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationSurfaceEnvironmentsSetProductionResponse {
    pub fn builder() -> MutationSurfaceEnvironmentsSetProductionResponseBuilder {
        <MutationSurfaceEnvironmentsSetProductionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationSurfaceEnvironmentsSetProductionResponseBuilder {
    success: Option<bool>,
}

impl MutationSurfaceEnvironmentsSetProductionResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationSurfaceEnvironmentsSetProductionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationSurfaceEnvironmentsSetProductionResponseBuilder::success)
    pub fn build(self) -> Result<MutationSurfaceEnvironmentsSetProductionResponse, BuildError> {
        Ok(MutationSurfaceEnvironmentsSetProductionResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
