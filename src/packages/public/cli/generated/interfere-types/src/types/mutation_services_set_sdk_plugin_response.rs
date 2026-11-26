pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationServicesSetSdkPluginResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationServicesSetSdkPluginResponse {
    pub fn builder() -> MutationServicesSetSdkPluginResponseBuilder {
        <MutationServicesSetSdkPluginResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationServicesSetSdkPluginResponseBuilder {
    success: Option<bool>,
}

impl MutationServicesSetSdkPluginResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationServicesSetSdkPluginResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationServicesSetSdkPluginResponseBuilder::success)
    pub fn build(self) -> Result<MutationServicesSetSdkPluginResponse, BuildError> {
        Ok(MutationServicesSetSdkPluginResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
