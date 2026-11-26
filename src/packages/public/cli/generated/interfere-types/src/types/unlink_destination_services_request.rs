pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnlinkDestinationServicesRequest {
    #[serde(default)]
    pub args: UnlinkDestinationServicesRequestArgs,
}

impl UnlinkDestinationServicesRequest {
    pub fn builder() -> UnlinkDestinationServicesRequestBuilder {
        <UnlinkDestinationServicesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnlinkDestinationServicesRequestBuilder {
    args: Option<UnlinkDestinationServicesRequestArgs>,
}

impl UnlinkDestinationServicesRequestBuilder {
    pub fn args(mut self, value: UnlinkDestinationServicesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnlinkDestinationServicesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UnlinkDestinationServicesRequestBuilder::args)
    pub fn build(self) -> Result<UnlinkDestinationServicesRequest, BuildError> {
        Ok(UnlinkDestinationServicesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

