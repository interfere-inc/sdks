pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TraceLogs {
    #[serde(default)]
    pub logs: Vec<TraceLog>,
    /// True when the trace has more log records than the requested limit
    #[serde(default)]
    pub truncated: bool,
}

impl TraceLogs {
    pub fn builder() -> TraceLogsBuilder {
        <TraceLogsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TraceLogsBuilder {
    logs: Option<Vec<TraceLog>>,
    truncated: Option<bool>,
}

impl TraceLogsBuilder {
    pub fn logs(mut self, value: Vec<TraceLog>) -> Self {
        self.logs = Some(value);
        self
    }

    pub fn truncated(mut self, value: bool) -> Self {
        self.truncated = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TraceLogs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`logs`](TraceLogsBuilder::logs)
    /// - [`truncated`](TraceLogsBuilder::truncated)
    pub fn build(self) -> Result<TraceLogs, BuildError> {
        Ok(TraceLogs {
            logs: self.logs.ok_or_else(|| BuildError::missing_field("logs"))?,
            truncated: self.truncated.ok_or_else(|| BuildError::missing_field("truncated"))?,
        })
    }
}
