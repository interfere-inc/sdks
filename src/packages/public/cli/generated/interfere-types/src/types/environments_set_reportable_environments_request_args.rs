pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetReportableEnvironmentsRequestArgs {
    /// Workspace environment name as reported by the SDK, lowercased, such as production.
    #[serde(default)]
    pub name: String,
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
    name: Option<String>,
    reportable: Option<bool>,
}

impl SetReportableEnvironmentsRequestArgsBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn reportable(mut self, value: bool) -> Self {
        self.reportable = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetReportableEnvironmentsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](SetReportableEnvironmentsRequestArgsBuilder::name)
    /// - [`reportable`](SetReportableEnvironmentsRequestArgsBuilder::reportable)
    pub fn build(self) -> Result<SetReportableEnvironmentsRequestArgs, BuildError> {
        Ok(SetReportableEnvironmentsRequestArgs {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            reportable: self.reportable.ok_or_else(|| BuildError::missing_field("reportable"))?,
        })
    }
}
