pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContentChartChart {
        TimelineEventContentChartChartAxis(TimelineEventContentChartChartAxis),

        TimelineEventContentChartChartSlices(TimelineEventContentChartChartSlices),
}

impl TimelineEventContentChartChart {
    pub fn is_timeline_event_content_chart_chart_axis(&self) -> bool {
        matches!(self, Self::TimelineEventContentChartChartAxis(_))
    }

    pub fn is_timeline_event_content_chart_chart_slices(&self) -> bool {
        matches!(self, Self::TimelineEventContentChartChartSlices(_))
    }


    pub fn as_timeline_event_content_chart_chart_axis(&self) -> Option<&TimelineEventContentChartChartAxis> {
        match self {
                    Self::TimelineEventContentChartChartAxis(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_chart_chart_axis(self) -> Option<TimelineEventContentChartChartAxis> {
        match self {
                    Self::TimelineEventContentChartChartAxis(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_chart_chart_slices(&self) -> Option<&TimelineEventContentChartChartSlices> {
        match self {
                    Self::TimelineEventContentChartChartSlices(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_chart_chart_slices(self) -> Option<TimelineEventContentChartChartSlices> {
        match self {
                    Self::TimelineEventContentChartChartSlices(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentChartChart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentChartChartAxis(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentChartChartSlices(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
