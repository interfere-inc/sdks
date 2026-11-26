pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationServicesUnlinkDestinationResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationServicesUnlinkDestinationResponse {
    pub fn builder() -> MutationServicesUnlinkDestinationResponseBuilder {
        <MutationServicesUnlinkDestinationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationServicesUnlinkDestinationResponseBuilder {
    success: Option<bool>,
}

impl MutationServicesUnlinkDestinationResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationServicesUnlinkDestinationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationServicesUnlinkDestinationResponseBuilder::success)
    pub fn build(self) -> Result<MutationServicesUnlinkDestinationResponse, BuildError> {
        Ok(MutationServicesUnlinkDestinationResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
