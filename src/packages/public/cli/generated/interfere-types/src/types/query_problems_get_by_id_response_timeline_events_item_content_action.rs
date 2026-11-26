pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentAction {
    pub action: QueryProblemsGetByIdResponseTimelineEventsItemContentActionAction,
    pub role: QueryProblemsGetByIdResponseTimelineEventsItemContentActionRole,
    #[serde(rename = "otherProblemId")]
    #[serde(default)]
    pub other_problem_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionBasis>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentAction {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentActionBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentActionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionBuilder {
    action: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionAction>,
    role: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionRole>,
    other_problem_id: Option<String>,
    basis: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionBasis>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionBuilder {
    pub fn action(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn role(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn other_problem_id(mut self, value: impl Into<String>) -> Self {
        self.other_problem_id = Some(value.into());
        self
    }

    pub fn basis(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionBasis) -> Self {
        self.basis = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentAction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionBuilder::action)
    /// - [`role`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionBuilder::role)
    /// - [`other_problem_id`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionBuilder::other_problem_id)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentAction, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentAction {
            action: self.action.ok_or_else(|| BuildError::missing_field("action"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            other_problem_id: self.other_problem_id.ok_or_else(|| BuildError::missing_field("other_problem_id"))?,
            basis: self.basis,
        })
    }
}
