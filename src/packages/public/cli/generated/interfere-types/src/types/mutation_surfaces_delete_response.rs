pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationSurfacesDeleteResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationSurfacesDeleteResponse {
    pub fn builder() -> MutationSurfacesDeleteResponseBuilder {
        <MutationSurfacesDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationSurfacesDeleteResponseBuilder {
    success: Option<bool>,
}

impl MutationSurfacesDeleteResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationSurfacesDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationSurfacesDeleteResponseBuilder::success)
    pub fn build(self) -> Result<MutationSurfacesDeleteResponse, BuildError> {
        Ok(MutationSurfacesDeleteResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
