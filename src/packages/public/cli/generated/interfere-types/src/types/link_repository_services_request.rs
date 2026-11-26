pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LinkRepositoryServicesRequest {
    #[serde(default)]
    pub args: LinkRepositoryServicesRequestArgs,
}

impl LinkRepositoryServicesRequest {
    pub fn builder() -> LinkRepositoryServicesRequestBuilder {
        <LinkRepositoryServicesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LinkRepositoryServicesRequestBuilder {
    args: Option<LinkRepositoryServicesRequestArgs>,
}

impl LinkRepositoryServicesRequestBuilder {
    pub fn args(mut self, value: LinkRepositoryServicesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LinkRepositoryServicesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](LinkRepositoryServicesRequestBuilder::args)
    pub fn build(self) -> Result<LinkRepositoryServicesRequest, BuildError> {
        Ok(LinkRepositoryServicesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

