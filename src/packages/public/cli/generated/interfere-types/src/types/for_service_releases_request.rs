pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ForServiceReleasesRequest {
    #[serde(default)]
    pub args: ForServiceReleasesRequestArgs,
}

impl ForServiceReleasesRequest {
    pub fn builder() -> ForServiceReleasesRequestBuilder {
        <ForServiceReleasesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForServiceReleasesRequestBuilder {
    args: Option<ForServiceReleasesRequestArgs>,
}

impl ForServiceReleasesRequestBuilder {
    pub fn args(mut self, value: ForServiceReleasesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ForServiceReleasesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](ForServiceReleasesRequestBuilder::args)
    pub fn build(self) -> Result<ForServiceReleasesRequest, BuildError> {
        Ok(ForServiceReleasesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

