pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ByIdIncludeDeletedDomainsRequest {
    #[serde(default)]
    pub args: ByIdIncludeDeletedDomainsRequestArgs,
}

impl ByIdIncludeDeletedDomainsRequest {
    pub fn builder() -> ByIdIncludeDeletedDomainsRequestBuilder {
        <ByIdIncludeDeletedDomainsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ByIdIncludeDeletedDomainsRequestBuilder {
    args: Option<ByIdIncludeDeletedDomainsRequestArgs>,
}

impl ByIdIncludeDeletedDomainsRequestBuilder {
    pub fn args(mut self, value: ByIdIncludeDeletedDomainsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ByIdIncludeDeletedDomainsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](ByIdIncludeDeletedDomainsRequestBuilder::args)
    pub fn build(self) -> Result<ByIdIncludeDeletedDomainsRequest, BuildError> {
        Ok(ByIdIncludeDeletedDomainsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

