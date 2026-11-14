pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimelineEventRequestContractScope {
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
}

impl TimelineEventRequestContractScope {
    pub fn builder() -> TimelineEventRequestContractScopeBuilder {
        <TimelineEventRequestContractScopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventRequestContractScopeBuilder {
    organization_id: Option<String>,
    problem_id: Option<String>,
}

impl TimelineEventRequestContractScopeBuilder {
    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventRequestContractScope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`organization_id`](TimelineEventRequestContractScopeBuilder::organization_id)
    /// - [`problem_id`](TimelineEventRequestContractScopeBuilder::problem_id)
    pub fn build(self) -> Result<TimelineEventRequestContractScope, BuildError> {
        Ok(TimelineEventRequestContractScope {
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
        })
    }
}
