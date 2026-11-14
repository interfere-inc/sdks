pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LatestBySurfaceReleasesRequest {
    #[serde(default)]
    pub args: LatestBySurfaceReleasesRequestArgs,
}

impl LatestBySurfaceReleasesRequest {
    pub fn builder() -> LatestBySurfaceReleasesRequestBuilder {
        <LatestBySurfaceReleasesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LatestBySurfaceReleasesRequestBuilder {
    args: Option<LatestBySurfaceReleasesRequestArgs>,
}

impl LatestBySurfaceReleasesRequestBuilder {
    pub fn args(mut self, value: LatestBySurfaceReleasesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LatestBySurfaceReleasesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](LatestBySurfaceReleasesRequestBuilder::args)
    pub fn build(self) -> Result<LatestBySurfaceReleasesRequest, BuildError> {
        Ok(LatestBySurfaceReleasesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

