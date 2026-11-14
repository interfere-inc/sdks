pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListForSurfaceReleasesRequest {
    #[serde(default)]
    pub args: ListForSurfaceReleasesRequestArgs,
}

impl ListForSurfaceReleasesRequest {
    pub fn builder() -> ListForSurfaceReleasesRequestBuilder {
        <ListForSurfaceReleasesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListForSurfaceReleasesRequestBuilder {
    args: Option<ListForSurfaceReleasesRequestArgs>,
}

impl ListForSurfaceReleasesRequestBuilder {
    pub fn args(mut self, value: ListForSurfaceReleasesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListForSurfaceReleasesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](ListForSurfaceReleasesRequestBuilder::args)
    pub fn build(self) -> Result<ListForSurfaceReleasesRequest, BuildError> {
        Ok(ListForSurfaceReleasesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

