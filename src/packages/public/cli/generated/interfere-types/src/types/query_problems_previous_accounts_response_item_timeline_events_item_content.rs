pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContent {
        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKind(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKind),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGap(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGap),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupId(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupId),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCategory(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCategory),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFour(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFour),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDescription(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDescription),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrection(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrection),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentAction(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentAction),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChart(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChart),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPriority(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPriority),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTen(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTen),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEleven(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEleven),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelve(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelve),

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

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_new_assignee_group_id(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupId(_))
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

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_correction(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrection(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_action(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentAction(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_chart(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChart(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_priority(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPriority(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_ten(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTen(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_eleven(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEleven(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_twelve(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelve(_))
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

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_new_assignee_group_id(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_new_assignee_group_id(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupId(value) => Some(value),
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

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_correction(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrection> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrection(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_correction(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrection> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrection(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_action(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentAction> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentAction(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_action(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentAction> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentAction(value) => Some(value),
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

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_priority(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPriority> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPriority(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_priority(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPriority> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPriority(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_ten(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTen> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTen(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_ten(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTen> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTen(value) => Some(value),
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

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_twelve(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelve> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelve(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_twelve(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelve> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelve(value) => Some(value),
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
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCategory(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFour(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDescription(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrection(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentAction(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChart(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPriority(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTen(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEleven(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelve(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentRelease(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissal(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTitle(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
