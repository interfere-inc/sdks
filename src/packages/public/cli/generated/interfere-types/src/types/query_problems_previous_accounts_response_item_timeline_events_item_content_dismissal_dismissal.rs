pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissal {
        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalZero(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalZero),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalProblemId(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalProblemId),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissal {
    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_dismissal_dismissal_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalZero(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_dismissal_dismissal_problem_id(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalProblemId(_))
    }


    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_dismissal_dismissal_zero(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalZero> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_dismissal_dismissal_zero(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalZero> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_dismissal_dismissal_problem_id(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalProblemId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalProblemId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_dismissal_dismissal_problem_id(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalProblemId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalProblemId(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalProblemId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
