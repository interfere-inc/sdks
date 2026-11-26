pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SetTypeServicesRequest {
    pub args: SetTypeServicesRequestArgs,
}

impl SetTypeServicesRequest {
    pub fn builder() -> SetTypeServicesRequestBuilder {
        <SetTypeServicesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetTypeServicesRequestBuilder {
    args: Option<SetTypeServicesRequestArgs>,
}

impl SetTypeServicesRequestBuilder {
    pub fn args(mut self, value: SetTypeServicesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetTypeServicesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](SetTypeServicesRequestBuilder::args)
    pub fn build(self) -> Result<SetTypeServicesRequest, BuildError> {
        Ok(SetTypeServicesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

