pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentAction {
    pub action: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionAction,
    pub role: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionRole,
    #[serde(rename = "otherProblemId")]
    #[serde(default)]
    pub other_problem_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionBasis>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentAction {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionBuilder {
    action: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionAction>,
    role: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionRole>,
    other_problem_id: Option<String>,
    basis: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionBasis>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionBuilder {
    pub fn action(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn role(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn other_problem_id(mut self, value: impl Into<String>) -> Self {
        self.other_problem_id = Some(value.into());
        self
    }

    pub fn basis(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionBasis) -> Self {
        self.basis = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentAction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionBuilder::action)
    /// - [`role`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionBuilder::role)
    /// - [`other_problem_id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionBuilder::other_problem_id)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentAction, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentAction {
            action: self.action.ok_or_else(|| BuildError::missing_field("action"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            other_problem_id: self.other_problem_id.ok_or_else(|| BuildError::missing_field("other_problem_id"))?,
            basis: self.basis,
        })
    }
}
