pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ByIdsServicesRequest {
    #[serde(default)]
    pub args: ByIdsServicesRequestArgs,
}

impl ByIdsServicesRequest {
    pub fn builder() -> ByIdsServicesRequestBuilder {
        <ByIdsServicesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ByIdsServicesRequestBuilder {
    args: Option<ByIdsServicesRequestArgs>,
}

impl ByIdsServicesRequestBuilder {
    pub fn args(mut self, value: ByIdsServicesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ByIdsServicesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](ByIdsServicesRequestBuilder::args)
    pub fn build(self) -> Result<ByIdsServicesRequest, BuildError> {
        Ok(ByIdsServicesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

