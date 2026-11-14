pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItem {
        QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemZero(QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemZero),

        QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOne(QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOne),

        QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemType(QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemType),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItem {
    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_chart_doc_content_item_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemZero(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_chart_doc_content_item_one(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOne(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_chart_doc_content_item_type(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemType(_))
    }


    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_chart_doc_content_item_zero(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_chart_doc_content_item_zero(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_chart_doc_content_item_one(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_chart_doc_content_item_one(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_chart_doc_content_item_type(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemType> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_chart_doc_content_item_type(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemType> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemType(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
