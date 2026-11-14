pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpanDurationHistory {
    #[serde(rename = "baselineMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub baseline_ms: Option<f64>,
    #[serde(default)]
    pub samples: Vec<f64>,
    /// ISO start time of each entry in samples, in the same order.
    #[serde(rename = "sampleStartedAt")]
    #[serde(default)]
    pub sample_started_at: Vec<DateTime<FixedOffset>>,
    #[serde(rename = "unknownSampleCount")]
    #[serde(default)]
    pub unknown_sample_count: i64,
}

impl SpanDurationHistory {
    pub fn builder() -> SpanDurationHistoryBuilder {
        <SpanDurationHistoryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpanDurationHistoryBuilder {
    baseline_ms: Option<f64>,
    samples: Option<Vec<f64>>,
    sample_started_at: Option<Vec<DateTime<FixedOffset>>>,
    unknown_sample_count: Option<i64>,
}

impl SpanDurationHistoryBuilder {
    pub fn baseline_ms(mut self, value: f64) -> Self {
        self.baseline_ms = Some(value);
        self
    }

    pub fn samples(mut self, value: Vec<f64>) -> Self {
        self.samples = Some(value);
        self
    }

    pub fn sample_started_at(mut self, value: Vec<DateTime<FixedOffset>>) -> Self {
        self.sample_started_at = Some(value);
        self
    }

    pub fn unknown_sample_count(mut self, value: i64) -> Self {
        self.unknown_sample_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpanDurationHistory`].
    /// This method will fail if any of the following fields are not set:
    /// - [`samples`](SpanDurationHistoryBuilder::samples)
    /// - [`sample_started_at`](SpanDurationHistoryBuilder::sample_started_at)
    /// - [`unknown_sample_count`](SpanDurationHistoryBuilder::unknown_sample_count)
    pub fn build(self) -> Result<SpanDurationHistory, BuildError> {
        Ok(SpanDurationHistory {
            baseline_ms: self.baseline_ms,
            samples: self.samples.ok_or_else(|| BuildError::missing_field("samples"))?,
            sample_started_at: self.sample_started_at.ok_or_else(|| BuildError::missing_field("sample_started_at"))?,
            unknown_sample_count: self.unknown_sample_count.ok_or_else(|| BuildError::missing_field("unknown_sample_count"))?,
        })
    }
}
