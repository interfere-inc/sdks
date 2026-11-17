pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetReportableEnvironmentsRequestArgs2 {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub reportable: bool,
}

impl SetReportableEnvironmentsRequestArgs2 {
    pub fn builder() -> SetReportableEnvironmentsRequestArgs2Builder {
        <SetReportableEnvironmentsRequestArgs2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetReportableEnvironmentsRequestArgs2Builder {
    id: Option<String>,
    reportable: Option<bool>,
}

impl SetReportableEnvironmentsRequestArgs2Builder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn reportable(mut self, value: bool) -> Self {
        self.reportable = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetReportableEnvironmentsRequestArgs2`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SetReportableEnvironmentsRequestArgs2Builder::id)
    /// - [`reportable`](SetReportableEnvironmentsRequestArgs2Builder::reportable)
    pub fn build(self) -> Result<SetReportableEnvironmentsRequestArgs2, BuildError> {
        Ok(SetReportableEnvironmentsRequestArgs2 {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            reportable: self.reportable.ok_or_else(|| BuildError::missing_field("reportable"))?,
        })
    }
}
