pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChart {
        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxis(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxis),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlices(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlices),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChart {
    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_chart_chart_axis(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxis(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_chart_chart_slices(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlices(_))
    }


    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_chart_chart_axis(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxis> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxis(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_chart_chart_axis(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxis> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxis(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_chart_chart_slices(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlices> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlices(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_chart_chart_slices(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlices> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlices(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxis(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartSlices(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
