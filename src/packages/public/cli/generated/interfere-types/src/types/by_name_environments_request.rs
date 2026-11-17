pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ByNameEnvironmentsRequest {
    #[serde(default)]
    pub args: ByNameEnvironmentsRequestArgs,
}

impl ByNameEnvironmentsRequest {
    pub fn builder() -> ByNameEnvironmentsRequestBuilder {
        <ByNameEnvironmentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ByNameEnvironmentsRequestBuilder {
    args: Option<ByNameEnvironmentsRequestArgs>,
}

impl ByNameEnvironmentsRequestBuilder {
    pub fn args(mut self, value: ByNameEnvironmentsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ByNameEnvironmentsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](ByNameEnvironmentsRequestBuilder::args)
    pub fn build(self) -> Result<ByNameEnvironmentsRequest, BuildError> {
        Ok(ByNameEnvironmentsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

