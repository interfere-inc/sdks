pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentChartChart {
        QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxis(QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxis),

        QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlices(QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlices),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentChartChart {
    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_chart_chart_axis(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxis(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_chart_chart_slices(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlices(_))
    }


    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_chart_chart_axis(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxis> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxis(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_chart_chart_axis(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxis> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxis(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_chart_chart_slices(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlices> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlices(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_chart_chart_slices(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlices> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlices(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentChartChart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxis(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlices(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
