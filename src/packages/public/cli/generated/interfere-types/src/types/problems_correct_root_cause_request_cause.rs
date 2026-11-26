pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CorrectRootCauseRequestCause {
    #[serde(default)]
    pub summary: String,
    #[serde(rename = "causeClass")]
    pub cause_class: CorrectRootCauseRequestCauseCauseClass,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<CorrectRootCauseRequestCauseLocationsItem>>,
}

impl CorrectRootCauseRequestCause {
    pub fn builder() -> CorrectRootCauseRequestCauseBuilder {
        <CorrectRootCauseRequestCauseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CorrectRootCauseRequestCauseBuilder {
    summary: Option<String>,
    cause_class: Option<CorrectRootCauseRequestCauseCauseClass>,
    locations: Option<Vec<CorrectRootCauseRequestCauseLocationsItem>>,
}

impl CorrectRootCauseRequestCauseBuilder {
    pub fn summary(mut self, value: impl Into<String>) -> Self {
        self.summary = Some(value.into());
        self
    }

    pub fn cause_class(mut self, value: CorrectRootCauseRequestCauseCauseClass) -> Self {
        self.cause_class = Some(value);
        self
    }

    pub fn locations(mut self, value: Vec<CorrectRootCauseRequestCauseLocationsItem>) -> Self {
        self.locations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CorrectRootCauseRequestCause`].
    /// This method will fail if any of the following fields are not set:
    /// - [`summary`](CorrectRootCauseRequestCauseBuilder::summary)
    /// - [`cause_class`](CorrectRootCauseRequestCauseBuilder::cause_class)
    pub fn build(self) -> Result<CorrectRootCauseRequestCause, BuildError> {
        Ok(CorrectRootCauseRequestCause {
            summary: self.summary.ok_or_else(|| BuildError::missing_field("summary"))?,
            cause_class: self.cause_class.ok_or_else(|| BuildError::missing_field("cause_class"))?,
            locations: self.locations,
        })
    }
}
