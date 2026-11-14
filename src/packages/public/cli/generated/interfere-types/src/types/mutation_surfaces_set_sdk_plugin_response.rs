pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationSurfacesSetSdkPluginResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationSurfacesSetSdkPluginResponse {
    pub fn builder() -> MutationSurfacesSetSdkPluginResponseBuilder {
        <MutationSurfacesSetSdkPluginResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationSurfacesSetSdkPluginResponseBuilder {
    success: Option<bool>,
}

impl MutationSurfacesSetSdkPluginResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationSurfacesSetSdkPluginResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationSurfacesSetSdkPluginResponseBuilder::success)
    pub fn build(self) -> Result<MutationSurfacesSetSdkPluginResponse, BuildError> {
        Ok(MutationSurfacesSetSdkPluginResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
