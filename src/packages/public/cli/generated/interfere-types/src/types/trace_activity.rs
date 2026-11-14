pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TraceActivity {
    #[serde(default)]
    pub available: bool,
    #[serde(default)]
    pub items: Vec<TraceActivityItemsItem>,
    #[serde(default)]
    pub truncated: bool,
    #[serde(rename = "windowStartUnixNano")]
    #[serde(default)]
    pub window_start_unix_nano: UInt64String,
    #[serde(rename = "windowEndUnixNano")]
    #[serde(default)]
    pub window_end_unix_nano: UInt64String,
}

impl TraceActivity {
    pub fn builder() -> TraceActivityBuilder {
        <TraceActivityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TraceActivityBuilder {
    available: Option<bool>,
    items: Option<Vec<TraceActivityItemsItem>>,
    truncated: Option<bool>,
    window_start_unix_nano: Option<UInt64String>,
    window_end_unix_nano: Option<UInt64String>,
}

impl TraceActivityBuilder {
    pub fn available(mut self, value: bool) -> Self {
        self.available = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<TraceActivityItemsItem>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn truncated(mut self, value: bool) -> Self {
        self.truncated = Some(value);
        self
    }

    pub fn window_start_unix_nano(mut self, value: UInt64String) -> Self {
        self.window_start_unix_nano = Some(value);
        self
    }

    pub fn window_end_unix_nano(mut self, value: UInt64String) -> Self {
        self.window_end_unix_nano = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TraceActivity`].
    /// This method will fail if any of the following fields are not set:
    /// - [`available`](TraceActivityBuilder::available)
    /// - [`items`](TraceActivityBuilder::items)
    /// - [`truncated`](TraceActivityBuilder::truncated)
    /// - [`window_start_unix_nano`](TraceActivityBuilder::window_start_unix_nano)
    /// - [`window_end_unix_nano`](TraceActivityBuilder::window_end_unix_nano)
    pub fn build(self) -> Result<TraceActivity, BuildError> {
        Ok(TraceActivity {
            available: self.available.ok_or_else(|| BuildError::missing_field("available"))?,
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            truncated: self.truncated.ok_or_else(|| BuildError::missing_field("truncated"))?,
            window_start_unix_nano: self.window_start_unix_nano.ok_or_else(|| BuildError::missing_field("window_start_unix_nano"))?,
            window_end_unix_nano: self.window_end_unix_nano.ok_or_else(|| BuildError::missing_field("window_end_unix_nano"))?,
        })
    }
}
