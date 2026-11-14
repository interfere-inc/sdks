pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PageTenantsRequest {
    #[serde(default)]
    pub args: PageTenantsRequestArgs,
}

impl PageTenantsRequest {
    pub fn builder() -> PageTenantsRequestBuilder {
        <PageTenantsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PageTenantsRequestBuilder {
    args: Option<PageTenantsRequestArgs>,
}

impl PageTenantsRequestBuilder {
    pub fn args(mut self, value: PageTenantsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PageTenantsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](PageTenantsRequestBuilder::args)
    pub fn build(self) -> Result<PageTenantsRequest, BuildError> {
        Ok(PageTenantsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

