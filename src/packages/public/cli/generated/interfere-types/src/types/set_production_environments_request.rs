pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetProductionEnvironmentsRequest {
    #[serde(default)]
    pub args: SetProductionEnvironmentsRequestArgs,
}

impl SetProductionEnvironmentsRequest {
    pub fn builder() -> SetProductionEnvironmentsRequestBuilder {
        <SetProductionEnvironmentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetProductionEnvironmentsRequestBuilder {
    args: Option<SetProductionEnvironmentsRequestArgs>,
}

impl SetProductionEnvironmentsRequestBuilder {
    pub fn args(mut self, value: SetProductionEnvironmentsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetProductionEnvironmentsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](SetProductionEnvironmentsRequestBuilder::args)
    pub fn build(self) -> Result<SetProductionEnvironmentsRequest, BuildError> {
        Ok(SetProductionEnvironmentsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

