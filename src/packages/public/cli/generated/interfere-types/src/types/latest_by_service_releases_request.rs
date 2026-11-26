pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LatestByServiceReleasesRequest {
    #[serde(default)]
    pub args: LatestByServiceReleasesRequestArgs,
}

impl LatestByServiceReleasesRequest {
    pub fn builder() -> LatestByServiceReleasesRequestBuilder {
        <LatestByServiceReleasesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LatestByServiceReleasesRequestBuilder {
    args: Option<LatestByServiceReleasesRequestArgs>,
}

impl LatestByServiceReleasesRequestBuilder {
    pub fn args(mut self, value: LatestByServiceReleasesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LatestByServiceReleasesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](LatestByServiceReleasesRequestBuilder::args)
    pub fn build(self) -> Result<LatestByServiceReleasesRequest, BuildError> {
        Ok(LatestByServiceReleasesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

