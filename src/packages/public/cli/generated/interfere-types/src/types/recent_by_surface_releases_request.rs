pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RecentBySurfaceReleasesRequest {
    #[serde(default)]
    pub args: RecentBySurfaceReleasesRequestArgs,
}

impl RecentBySurfaceReleasesRequest {
    pub fn builder() -> RecentBySurfaceReleasesRequestBuilder {
        <RecentBySurfaceReleasesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecentBySurfaceReleasesRequestBuilder {
    args: Option<RecentBySurfaceReleasesRequestArgs>,
}

impl RecentBySurfaceReleasesRequestBuilder {
    pub fn args(mut self, value: RecentBySurfaceReleasesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecentBySurfaceReleasesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](RecentBySurfaceReleasesRequestBuilder::args)
    pub fn build(self) -> Result<RecentBySurfaceReleasesRequest, BuildError> {
        Ok(RecentBySurfaceReleasesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

