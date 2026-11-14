pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetReportableEnvironmentsRequestArgs {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub reportable: bool,
}

impl SetReportableEnvironmentsRequestArgs {
    pub fn builder() -> SetReportableEnvironmentsRequestArgsBuilder {
        <SetReportableEnvironmentsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetReportableEnvironmentsRequestArgsBuilder {
    id: Option<String>,
    reportable: Option<bool>,
}

impl SetReportableEnvironmentsRequestArgsBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn reportable(mut self, value: bool) -> Self {
        self.reportable = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetReportableEnvironmentsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SetReportableEnvironmentsRequestArgsBuilder::id)
    /// - [`reportable`](SetReportableEnvironmentsRequestArgsBuilder::reportable)
    pub fn build(self) -> Result<SetReportableEnvironmentsRequestArgs, BuildError> {
        Ok(SetReportableEnvironmentsRequestArgs {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            reportable: self.reportable.ok_or_else(|| BuildError::missing_field("reportable"))?,
        })
    }
}
