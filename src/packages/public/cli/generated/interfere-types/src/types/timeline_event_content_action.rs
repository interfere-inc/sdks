pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentAction {
    pub action: TimelineEventContentActionAction,
    pub role: TimelineEventContentActionRole,
    #[serde(rename = "otherProblemId")]
    #[serde(default)]
    pub other_problem_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis: Option<TimelineEventContentActionBasis>,
}

impl TimelineEventContentAction {
    pub fn builder() -> TimelineEventContentActionBuilder {
        <TimelineEventContentActionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentActionBuilder {
    action: Option<TimelineEventContentActionAction>,
    role: Option<TimelineEventContentActionRole>,
    other_problem_id: Option<String>,
    basis: Option<TimelineEventContentActionBasis>,
}

impl TimelineEventContentActionBuilder {
    pub fn action(mut self, value: TimelineEventContentActionAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn role(mut self, value: TimelineEventContentActionRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn other_problem_id(mut self, value: impl Into<String>) -> Self {
        self.other_problem_id = Some(value.into());
        self
    }

    pub fn basis(mut self, value: TimelineEventContentActionBasis) -> Self {
        self.basis = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentAction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](TimelineEventContentActionBuilder::action)
    /// - [`role`](TimelineEventContentActionBuilder::role)
    /// - [`other_problem_id`](TimelineEventContentActionBuilder::other_problem_id)
    pub fn build(self) -> Result<TimelineEventContentAction, BuildError> {
        Ok(TimelineEventContentAction {
            action: self.action.ok_or_else(|| BuildError::missing_field("action"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            other_problem_id: self.other_problem_id.ok_or_else(|| BuildError::missing_field("other_problem_id"))?,
            basis: self.basis,
        })
    }
}
