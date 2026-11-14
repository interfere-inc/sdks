pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SessionReplayIndexResponseChunksItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "windowId")]
    #[serde(default)]
    pub window_id: String,
    #[serde(rename = "segmentId")]
    #[serde(default)]
    pub segment_id: i64,
    #[serde(rename = "firstEventTs")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub first_event_ts: f64,
    #[serde(rename = "lastEventTs")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub last_event_ts: f64,
    #[serde(rename = "activityCount")]
    #[serde(default)]
    pub activity_count: i64,
    #[serde(rename = "hasFullSnapshot")]
    #[serde(default)]
    pub has_full_snapshot: bool,
}

impl SessionReplayIndexResponseChunksItem {
    pub fn builder() -> SessionReplayIndexResponseChunksItemBuilder {
        <SessionReplayIndexResponseChunksItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionReplayIndexResponseChunksItemBuilder {
    id: Option<String>,
    window_id: Option<String>,
    segment_id: Option<i64>,
    first_event_ts: Option<f64>,
    last_event_ts: Option<f64>,
    activity_count: Option<i64>,
    has_full_snapshot: Option<bool>,
}

impl SessionReplayIndexResponseChunksItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn window_id(mut self, value: impl Into<String>) -> Self {
        self.window_id = Some(value.into());
        self
    }

    pub fn segment_id(mut self, value: i64) -> Self {
        self.segment_id = Some(value);
        self
    }

    pub fn first_event_ts(mut self, value: f64) -> Self {
        self.first_event_ts = Some(value);
        self
    }

    pub fn last_event_ts(mut self, value: f64) -> Self {
        self.last_event_ts = Some(value);
        self
    }

    pub fn activity_count(mut self, value: i64) -> Self {
        self.activity_count = Some(value);
        self
    }

    pub fn has_full_snapshot(mut self, value: bool) -> Self {
        self.has_full_snapshot = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SessionReplayIndexResponseChunksItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SessionReplayIndexResponseChunksItemBuilder::id)
    /// - [`window_id`](SessionReplayIndexResponseChunksItemBuilder::window_id)
    /// - [`segment_id`](SessionReplayIndexResponseChunksItemBuilder::segment_id)
    /// - [`first_event_ts`](SessionReplayIndexResponseChunksItemBuilder::first_event_ts)
    /// - [`last_event_ts`](SessionReplayIndexResponseChunksItemBuilder::last_event_ts)
    /// - [`activity_count`](SessionReplayIndexResponseChunksItemBuilder::activity_count)
    /// - [`has_full_snapshot`](SessionReplayIndexResponseChunksItemBuilder::has_full_snapshot)
    pub fn build(self) -> Result<SessionReplayIndexResponseChunksItem, BuildError> {
        Ok(SessionReplayIndexResponseChunksItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            window_id: self.window_id.ok_or_else(|| BuildError::missing_field("window_id"))?,
            segment_id: self.segment_id.ok_or_else(|| BuildError::missing_field("segment_id"))?,
            first_event_ts: self.first_event_ts.ok_or_else(|| BuildError::missing_field("first_event_ts"))?,
            last_event_ts: self.last_event_ts.ok_or_else(|| BuildError::missing_field("last_event_ts"))?,
            activity_count: self.activity_count.ok_or_else(|| BuildError::missing_field("activity_count"))?,
            has_full_snapshot: self.has_full_snapshot.ok_or_else(|| BuildError::missing_field("has_full_snapshot"))?,
        })
    }
}
