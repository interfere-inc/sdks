pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LinkDestinationServicesRequest {
    #[serde(default)]
    pub args: LinkDestinationServicesRequestArgs,
}

impl LinkDestinationServicesRequest {
    pub fn builder() -> LinkDestinationServicesRequestBuilder {
        <LinkDestinationServicesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LinkDestinationServicesRequestBuilder {
    args: Option<LinkDestinationServicesRequestArgs>,
}

impl LinkDestinationServicesRequestBuilder {
    pub fn args(mut self, value: LinkDestinationServicesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LinkDestinationServicesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](LinkDestinationServicesRequestBuilder::args)
    pub fn build(self) -> Result<LinkDestinationServicesRequest, BuildError> {
        Ok(LinkDestinationServicesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

