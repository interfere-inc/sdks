pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublication {
        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasis(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasis),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReason(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReason),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublication {
    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_evidence_gap_publication_basis(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasis(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_evidence_gap_publication_reason(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReason(_))
    }


    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_evidence_gap_publication_basis(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasis> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasis(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_evidence_gap_publication_basis(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasis> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasis(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_evidence_gap_publication_reason(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReason> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReason(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_evidence_gap_publication_reason(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReason> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReason(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasis(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReason(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
