pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContent {
        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKind(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKind),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGap(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGap),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeUserId(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeUserId),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCategory(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCategory),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFour(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFour),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDescription(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDescription),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSix(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSix),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChart(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChart),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasis(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasis),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNine(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNine),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCosineScore(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCosineScore),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEleven(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEleven),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentRelease(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentRelease),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissal(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissal),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTitle(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTitle),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContent {
    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKind(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_evidence_gap(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGap(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_new_assignee_user_id(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeUserId(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_category(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCategory(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_four(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFour(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_description(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDescription(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_six(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSix(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_chart(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChart(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_basis(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasis(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_nine(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNine(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_cosine_score(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCosineScore(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_eleven(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEleven(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_release(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentRelease(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_dismissal(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissal(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_title(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTitle(_))
    }


    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKind> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKind(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKind> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKind(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_evidence_gap(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGap> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGap(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_evidence_gap(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGap> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGap(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_new_assignee_user_id(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeUserId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeUserId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_new_assignee_user_id(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeUserId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeUserId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_category(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCategory> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCategory(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_category(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCategory> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCategory(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_four(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFour> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFour(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_four(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFour> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFour(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_description(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDescription> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDescription(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_description(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDescription> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDescription(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_six(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSix> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSix(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_six(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSix> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSix(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_chart(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChart> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChart(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_chart(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChart> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChart(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_basis(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasis> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasis(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_basis(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasis> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasis(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_nine(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNine> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_nine(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNine> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_cosine_score(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCosineScore> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCosineScore(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_cosine_score(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCosineScore> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCosineScore(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_eleven(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEleven> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEleven(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_eleven(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEleven> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEleven(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_release(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentRelease> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentRelease(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_release(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentRelease> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentRelease(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_dismissal(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissal> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissal(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_dismissal(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissal> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissal(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_title(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTitle> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTitle(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_title(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTitle> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTitle(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKind(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGap(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeUserId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCategory(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFour(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDescription(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSix(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChart(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasis(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCosineScore(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEleven(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentRelease(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissal(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTitle(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
