pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationServicesLinkDestinationResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationServicesLinkDestinationResponse {
    pub fn builder() -> MutationServicesLinkDestinationResponseBuilder {
        <MutationServicesLinkDestinationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationServicesLinkDestinationResponseBuilder {
    success: Option<bool>,
}

impl MutationServicesLinkDestinationResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationServicesLinkDestinationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationServicesLinkDestinationResponseBuilder::success)
    pub fn build(self) -> Result<MutationServicesLinkDestinationResponse, BuildError> {
        Ok(MutationServicesLinkDestinationResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
