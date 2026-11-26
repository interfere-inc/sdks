pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnarchiveServicesRequest {
    #[serde(default)]
    pub args: UnarchiveServicesRequestArgs,
}

impl UnarchiveServicesRequest {
    pub fn builder() -> UnarchiveServicesRequestBuilder {
        <UnarchiveServicesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnarchiveServicesRequestBuilder {
    args: Option<UnarchiveServicesRequestArgs>,
}

impl UnarchiveServicesRequestBuilder {
    pub fn args(mut self, value: UnarchiveServicesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnarchiveServicesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UnarchiveServicesRequestBuilder::args)
    pub fn build(self) -> Result<UnarchiveServicesRequest, BuildError> {
        Ok(UnarchiveServicesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

