pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetBySlugIncludeDeletedSurfacesRequest {
    #[serde(default)]
    pub args: GetBySlugIncludeDeletedSurfacesRequestArgs,
}

impl GetBySlugIncludeDeletedSurfacesRequest {
    pub fn builder() -> GetBySlugIncludeDeletedSurfacesRequestBuilder {
        <GetBySlugIncludeDeletedSurfacesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetBySlugIncludeDeletedSurfacesRequestBuilder {
    args: Option<GetBySlugIncludeDeletedSurfacesRequestArgs>,
}

impl GetBySlugIncludeDeletedSurfacesRequestBuilder {
    pub fn args(mut self, value: GetBySlugIncludeDeletedSurfacesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetBySlugIncludeDeletedSurfacesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](GetBySlugIncludeDeletedSurfacesRequestBuilder::args)
    pub fn build(self) -> Result<GetBySlugIncludeDeletedSurfacesRequest, BuildError> {
        Ok(GetBySlugIncludeDeletedSurfacesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

