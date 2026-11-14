pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ByEndUserDevicesRequest {
    #[serde(default)]
    pub args: ByEndUserDevicesRequestArgs,
}

impl ByEndUserDevicesRequest {
    pub fn builder() -> ByEndUserDevicesRequestBuilder {
        <ByEndUserDevicesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ByEndUserDevicesRequestBuilder {
    args: Option<ByEndUserDevicesRequestArgs>,
}

impl ByEndUserDevicesRequestBuilder {
    pub fn args(mut self, value: ByEndUserDevicesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ByEndUserDevicesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](ByEndUserDevicesRequestBuilder::args)
    pub fn build(self) -> Result<ByEndUserDevicesRequest, BuildError> {
        Ok(ByEndUserDevicesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

