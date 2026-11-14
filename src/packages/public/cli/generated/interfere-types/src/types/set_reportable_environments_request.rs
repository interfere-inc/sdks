pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetReportableEnvironmentsRequest {
    #[serde(default)]
    pub args: SetReportableEnvironmentsRequestArgs,
}

impl SetReportableEnvironmentsRequest {
    pub fn builder() -> SetReportableEnvironmentsRequestBuilder {
        <SetReportableEnvironmentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetReportableEnvironmentsRequestBuilder {
    args: Option<SetReportableEnvironmentsRequestArgs>,
}

impl SetReportableEnvironmentsRequestBuilder {
    pub fn args(mut self, value: SetReportableEnvironmentsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetReportableEnvironmentsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](SetReportableEnvironmentsRequestBuilder::args)
    pub fn build(self) -> Result<SetReportableEnvironmentsRequest, BuildError> {
        Ok(SetReportableEnvironmentsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

