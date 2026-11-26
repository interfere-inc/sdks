pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationServicesSetWebsiteSettingsResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationServicesSetWebsiteSettingsResponse {
    pub fn builder() -> MutationServicesSetWebsiteSettingsResponseBuilder {
        <MutationServicesSetWebsiteSettingsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationServicesSetWebsiteSettingsResponseBuilder {
    success: Option<bool>,
}

impl MutationServicesSetWebsiteSettingsResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationServicesSetWebsiteSettingsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationServicesSetWebsiteSettingsResponseBuilder::success)
    pub fn build(self) -> Result<MutationServicesSetWebsiteSettingsResponse, BuildError> {
        Ok(MutationServicesSetWebsiteSettingsResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
