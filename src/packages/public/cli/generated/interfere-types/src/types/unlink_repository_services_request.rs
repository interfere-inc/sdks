pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnlinkRepositoryServicesRequest {
    #[serde(default)]
    pub args: UnlinkRepositoryServicesRequestArgs,
}

impl UnlinkRepositoryServicesRequest {
    pub fn builder() -> UnlinkRepositoryServicesRequestBuilder {
        <UnlinkRepositoryServicesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnlinkRepositoryServicesRequestBuilder {
    args: Option<UnlinkRepositoryServicesRequestArgs>,
}

impl UnlinkRepositoryServicesRequestBuilder {
    pub fn args(mut self, value: UnlinkRepositoryServicesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnlinkRepositoryServicesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UnlinkRepositoryServicesRequestBuilder::args)
    pub fn build(self) -> Result<UnlinkRepositoryServicesRequest, BuildError> {
        Ok(UnlinkRepositoryServicesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

