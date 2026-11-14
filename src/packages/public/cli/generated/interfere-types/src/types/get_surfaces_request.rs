pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSurfacesRequest {
    #[serde(default)]
    pub args: GetSurfacesRequestArgs,
}

impl GetSurfacesRequest {
    pub fn builder() -> GetSurfacesRequestBuilder {
        <GetSurfacesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSurfacesRequestBuilder {
    args: Option<GetSurfacesRequestArgs>,
}

impl GetSurfacesRequestBuilder {
    pub fn args(mut self, value: GetSurfacesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSurfacesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](GetSurfacesRequestBuilder::args)
    pub fn build(self) -> Result<GetSurfacesRequest, BuildError> {
        Ok(GetSurfacesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

