pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RecentByServiceReleasesRequest {
    #[serde(default)]
    pub args: RecentByServiceReleasesRequestArgs,
}

impl RecentByServiceReleasesRequest {
    pub fn builder() -> RecentByServiceReleasesRequestBuilder {
        <RecentByServiceReleasesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecentByServiceReleasesRequestBuilder {
    args: Option<RecentByServiceReleasesRequestArgs>,
}

impl RecentByServiceReleasesRequestBuilder {
    pub fn args(mut self, value: RecentByServiceReleasesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecentByServiceReleasesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](RecentByServiceReleasesRequestBuilder::args)
    pub fn build(self) -> Result<RecentByServiceReleasesRequest, BuildError> {
        Ok(RecentByServiceReleasesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

