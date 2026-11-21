pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContent {
        QueryProblemsGetByIdResponseTimelineEventsItemContentActionKind(QueryProblemsGetByIdResponseTimelineEventsItemContentActionKind),

        QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGap(QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGap),

        QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeGroupId(QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeGroupId),

        QueryProblemsGetByIdResponseTimelineEventsItemContentCategory(QueryProblemsGetByIdResponseTimelineEventsItemContentCategory),

        QueryProblemsGetByIdResponseTimelineEventsItemContentFour(QueryProblemsGetByIdResponseTimelineEventsItemContentFour),

        QueryProblemsGetByIdResponseTimelineEventsItemContentDescription(QueryProblemsGetByIdResponseTimelineEventsItemContentDescription),

        QueryProblemsGetByIdResponseTimelineEventsItemContentSix(QueryProblemsGetByIdResponseTimelineEventsItemContentSix),

        QueryProblemsGetByIdResponseTimelineEventsItemContentChart(QueryProblemsGetByIdResponseTimelineEventsItemContentChart),

        QueryProblemsGetByIdResponseTimelineEventsItemContentBasis(QueryProblemsGetByIdResponseTimelineEventsItemContentBasis),

        QueryProblemsGetByIdResponseTimelineEventsItemContentNine(QueryProblemsGetByIdResponseTimelineEventsItemContentNine),

        QueryProblemsGetByIdResponseTimelineEventsItemContentCosineScore(QueryProblemsGetByIdResponseTimelineEventsItemContentCosineScore),

        QueryProblemsGetByIdResponseTimelineEventsItemContentEleven(QueryProblemsGetByIdResponseTimelineEventsItemContentEleven),

        QueryProblemsGetByIdResponseTimelineEventsItemContentRelease(QueryProblemsGetByIdResponseTimelineEventsItemContentRelease),

        QueryProblemsGetByIdResponseTimelineEventsItemContentDismissal(QueryProblemsGetByIdResponseTimelineEventsItemContentDismissal),

        QueryProblemsGetByIdResponseTimelineEventsItemContentTitle(QueryProblemsGetByIdResponseTimelineEventsItemContentTitle),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContent {
    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_action_kind(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKind(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_evidence_gap(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGap(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_new_assignee_group_id(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeGroupId(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_category(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentCategory(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_four(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentFour(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_description(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentDescription(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_six(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentSix(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_chart(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentChart(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_basis(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentBasis(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_nine(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentNine(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_cosine_score(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentCosineScore(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_eleven(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEleven(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_release(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentRelease(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_dismissal(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentDismissal(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_title(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTitle(_))
    }


    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_action_kind(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentActionKind> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKind(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_action_kind(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKind> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKind(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_evidence_gap(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGap> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGap(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_evidence_gap(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGap> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGap(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_new_assignee_group_id(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeGroupId> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeGroupId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_new_assignee_group_id(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeGroupId> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeGroupId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_category(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentCategory> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentCategory(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_category(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentCategory> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentCategory(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_four(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentFour> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentFour(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_four(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentFour> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentFour(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_description(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentDescription> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentDescription(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_description(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentDescription> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentDescription(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_six(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentSix> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentSix(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_six(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentSix> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentSix(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_chart(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentChart> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentChart(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_chart(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentChart> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentChart(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_basis(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentBasis> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentBasis(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_basis(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentBasis> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentBasis(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_nine(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentNine> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_nine(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentNine> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_cosine_score(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentCosineScore> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentCosineScore(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_cosine_score(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentCosineScore> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentCosineScore(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_eleven(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentEleven> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEleven(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_eleven(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEleven> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEleven(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_release(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentRelease> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentRelease(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_release(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentRelease> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentRelease(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_dismissal(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentDismissal> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentDismissal(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_dismissal(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissal> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentDismissal(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_title(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentTitle> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTitle(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_title(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentTitle> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTitle(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKind(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGap(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeGroupId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentCategory(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentFour(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentDescription(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentSix(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentChart(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentBasis(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentNine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentCosineScore(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEleven(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentRelease(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentDismissal(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTitle(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
