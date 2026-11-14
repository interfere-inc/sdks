pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDomainsRequest {
    #[serde(default)]
    pub args: GetDomainsRequestArgs,
}

impl GetDomainsRequest {
    pub fn builder() -> GetDomainsRequestBuilder {
        <GetDomainsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDomainsRequestBuilder {
    args: Option<GetDomainsRequestArgs>,
}

impl GetDomainsRequestBuilder {
    pub fn args(mut self, value: GetDomainsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetDomainsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](GetDomainsRequestBuilder::args)
    pub fn build(self) -> Result<GetDomainsRequest, BuildError> {
        Ok(GetDomainsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

