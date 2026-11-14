pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContentChartDocContentItem {
        TimelineEventContentChartDocContentItemZero(TimelineEventContentChartDocContentItemZero),

        TimelineEventContentChartDocContentItemOne(TimelineEventContentChartDocContentItemOne),

        TimelineEventContentChartDocContentItemType(TimelineEventContentChartDocContentItemType),
}

impl TimelineEventContentChartDocContentItem {
    pub fn is_timeline_event_content_chart_doc_content_item_zero(&self) -> bool {
        matches!(self, Self::TimelineEventContentChartDocContentItemZero(_))
    }

    pub fn is_timeline_event_content_chart_doc_content_item_one(&self) -> bool {
        matches!(self, Self::TimelineEventContentChartDocContentItemOne(_))
    }

    pub fn is_timeline_event_content_chart_doc_content_item_type(&self) -> bool {
        matches!(self, Self::TimelineEventContentChartDocContentItemType(_))
    }


    pub fn as_timeline_event_content_chart_doc_content_item_zero(&self) -> Option<&TimelineEventContentChartDocContentItemZero> {
        match self {
                    Self::TimelineEventContentChartDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_chart_doc_content_item_zero(self) -> Option<TimelineEventContentChartDocContentItemZero> {
        match self {
                    Self::TimelineEventContentChartDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_chart_doc_content_item_one(&self) -> Option<&TimelineEventContentChartDocContentItemOne> {
        match self {
                    Self::TimelineEventContentChartDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_chart_doc_content_item_one(self) -> Option<TimelineEventContentChartDocContentItemOne> {
        match self {
                    Self::TimelineEventContentChartDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_chart_doc_content_item_type(&self) -> Option<&TimelineEventContentChartDocContentItemType> {
        match self {
                    Self::TimelineEventContentChartDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_chart_doc_content_item_type(self) -> Option<TimelineEventContentChartDocContentItemType> {
        match self {
                    Self::TimelineEventContentChartDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentChartDocContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentChartDocContentItemZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentChartDocContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentChartDocContentItemType(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
