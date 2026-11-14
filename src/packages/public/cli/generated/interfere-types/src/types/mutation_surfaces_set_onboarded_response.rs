pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationSurfacesSetOnboardedResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationSurfacesSetOnboardedResponse {
    pub fn builder() -> MutationSurfacesSetOnboardedResponseBuilder {
        <MutationSurfacesSetOnboardedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationSurfacesSetOnboardedResponseBuilder {
    success: Option<bool>,
}

impl MutationSurfacesSetOnboardedResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationSurfacesSetOnboardedResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationSurfacesSetOnboardedResponseBuilder::success)
    pub fn build(self) -> Result<MutationSurfacesSetOnboardedResponse, BuildError> {
        Ok(MutationSurfacesSetOnboardedResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
