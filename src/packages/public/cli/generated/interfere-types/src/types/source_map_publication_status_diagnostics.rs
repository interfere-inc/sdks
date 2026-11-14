pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SourceMapPublicationStatusDiagnostics {
    #[serde(default)]
    pub attempt: i64,
    #[serde(rename = "scheduledAtMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at_ms: Option<i64>,
    #[serde(rename = "startedAtMs")]
    #[serde(default)]
    pub started_at_ms: i64,
    #[serde(rename = "completedAtMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at_ms: Option<i64>,
    pub phase: SourceMapPublicationStatusDiagnosticsPhase,
    #[serde(rename = "publishedAtStart")]
    #[serde(default)]
    pub published_at_start: i64,
    #[serde(rename = "progressObservedAtMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_observed_at_ms: Option<i64>,
    #[serde(rename = "nativeRetryCount")]
    #[serde(default)]
    pub native_retry_count: i64,
    #[serde(rename = "interruptedAttempts")]
    #[serde(default)]
    pub interrupted_attempts: i64,
    #[serde(rename = "longestAttempt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longest_attempt: Option<SourceMapPublicationStatusDiagnosticsLongestAttempt>,
    #[serde(rename = "longestAlarmDelay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longest_alarm_delay: Option<SourceMapPublicationStatusDiagnosticsLongestAlarmDelay>,
    #[serde(rename = "nextAlarmAtMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_alarm_at_ms: Option<i64>,
    #[serde(rename = "lastFailure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure: Option<SourceMapPublicationStatusDiagnosticsLastFailure>,
}

impl SourceMapPublicationStatusDiagnostics {
    pub fn builder() -> SourceMapPublicationStatusDiagnosticsBuilder {
        <SourceMapPublicationStatusDiagnosticsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SourceMapPublicationStatusDiagnosticsBuilder {
    attempt: Option<i64>,
    scheduled_at_ms: Option<i64>,
    started_at_ms: Option<i64>,
    completed_at_ms: Option<i64>,
    phase: Option<SourceMapPublicationStatusDiagnosticsPhase>,
    published_at_start: Option<i64>,
    progress_observed_at_ms: Option<i64>,
    native_retry_count: Option<i64>,
    interrupted_attempts: Option<i64>,
    longest_attempt: Option<SourceMapPublicationStatusDiagnosticsLongestAttempt>,
    longest_alarm_delay: Option<SourceMapPublicationStatusDiagnosticsLongestAlarmDelay>,
    next_alarm_at_ms: Option<i64>,
    last_failure: Option<SourceMapPublicationStatusDiagnosticsLastFailure>,
}

impl SourceMapPublicationStatusDiagnosticsBuilder {
    pub fn attempt(mut self, value: i64) -> Self {
        self.attempt = Some(value);
        self
    }

    pub fn scheduled_at_ms(mut self, value: i64) -> Self {
        self.scheduled_at_ms = Some(value);
        self
    }

    pub fn started_at_ms(mut self, value: i64) -> Self {
        self.started_at_ms = Some(value);
        self
    }

    pub fn completed_at_ms(mut self, value: i64) -> Self {
        self.completed_at_ms = Some(value);
        self
    }

    pub fn phase(mut self, value: SourceMapPublicationStatusDiagnosticsPhase) -> Self {
        self.phase = Some(value);
        self
    }

    pub fn published_at_start(mut self, value: i64) -> Self {
        self.published_at_start = Some(value);
        self
    }

    pub fn progress_observed_at_ms(mut self, value: i64) -> Self {
        self.progress_observed_at_ms = Some(value);
        self
    }

    pub fn native_retry_count(mut self, value: i64) -> Self {
        self.native_retry_count = Some(value);
        self
    }

    pub fn interrupted_attempts(mut self, value: i64) -> Self {
        self.interrupted_attempts = Some(value);
        self
    }

    pub fn longest_attempt(mut self, value: SourceMapPublicationStatusDiagnosticsLongestAttempt) -> Self {
        self.longest_attempt = Some(value);
        self
    }

    pub fn longest_alarm_delay(mut self, value: SourceMapPublicationStatusDiagnosticsLongestAlarmDelay) -> Self {
        self.longest_alarm_delay = Some(value);
        self
    }

    pub fn next_alarm_at_ms(mut self, value: i64) -> Self {
        self.next_alarm_at_ms = Some(value);
        self
    }

    pub fn last_failure(mut self, value: SourceMapPublicationStatusDiagnosticsLastFailure) -> Self {
        self.last_failure = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SourceMapPublicationStatusDiagnostics`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attempt`](SourceMapPublicationStatusDiagnosticsBuilder::attempt)
    /// - [`started_at_ms`](SourceMapPublicationStatusDiagnosticsBuilder::started_at_ms)
    /// - [`phase`](SourceMapPublicationStatusDiagnosticsBuilder::phase)
    /// - [`published_at_start`](SourceMapPublicationStatusDiagnosticsBuilder::published_at_start)
    /// - [`native_retry_count`](SourceMapPublicationStatusDiagnosticsBuilder::native_retry_count)
    /// - [`interrupted_attempts`](SourceMapPublicationStatusDiagnosticsBuilder::interrupted_attempts)
    pub fn build(self) -> Result<SourceMapPublicationStatusDiagnostics, BuildError> {
        Ok(SourceMapPublicationStatusDiagnostics {
            attempt: self.attempt.ok_or_else(|| BuildError::missing_field("attempt"))?,
            scheduled_at_ms: self.scheduled_at_ms,
            started_at_ms: self.started_at_ms.ok_or_else(|| BuildError::missing_field("started_at_ms"))?,
            completed_at_ms: self.completed_at_ms,
            phase: self.phase.ok_or_else(|| BuildError::missing_field("phase"))?,
            published_at_start: self.published_at_start.ok_or_else(|| BuildError::missing_field("published_at_start"))?,
            progress_observed_at_ms: self.progress_observed_at_ms,
            native_retry_count: self.native_retry_count.ok_or_else(|| BuildError::missing_field("native_retry_count"))?,
            interrupted_attempts: self.interrupted_attempts.ok_or_else(|| BuildError::missing_field("interrupted_attempts"))?,
            longest_attempt: self.longest_attempt,
            longest_alarm_delay: self.longest_alarm_delay,
            next_alarm_at_ms: self.next_alarm_at_ms,
            last_failure: self.last_failure,
        })
    }
}
