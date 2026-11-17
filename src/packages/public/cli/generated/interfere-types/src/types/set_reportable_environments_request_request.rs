pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetReportableEnvironmentsRequest2 {
    #[serde(default)]
    pub args: SetReportableEnvironmentsRequestArgs2,
}

impl SetReportableEnvironmentsRequest2 {
    pub fn builder() -> SetReportableEnvironmentsRequest2Builder {
        <SetReportableEnvironmentsRequest2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetReportableEnvironmentsRequest2Builder {
    args: Option<SetReportableEnvironmentsRequestArgs2>,
}

impl SetReportableEnvironmentsRequest2Builder {
    pub fn args(mut self, value: SetReportableEnvironmentsRequestArgs2) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetReportableEnvironmentsRequest2`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](SetReportableEnvironmentsRequest2Builder::args)
    pub fn build(self) -> Result<SetReportableEnvironmentsRequest2, BuildError> {
        Ok(SetReportableEnvironmentsRequest2 {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

