pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FindingOccurrence {
    #[serde(rename = "entryName")]
    #[serde(default)]
    pub entry_name: String,
    #[serde(rename = "entryService")]
    #[serde(default)]
    pub entry_service: String,
    #[serde(default)]
    pub environment: String,
    #[serde(rename = "errorCount")]
    #[serde(default)]
    pub error_count: i64,
    /// UTC timestamp formatted as YYYY-MM-DD HH:mm:ss.
    #[serde(rename = "firstAt")]
    #[serde(default)]
    pub first_at: String,
    #[serde(rename = "firstAtUnixNano")]
    #[serde(default)]
    pub first_at_unix_nano: UInt64String,
    #[serde(rename = "hasEnrichedFrames")]
    #[serde(default)]
    pub has_enriched_frames: bool,
    #[serde(rename = "lastAtUnixNano")]
    #[serde(default)]
    pub last_at_unix_nano: UInt64String,
    #[serde(rename = "occurrencesInTrace")]
    #[serde(default)]
    pub occurrences_in_trace: i64,
    #[serde(rename = "releaseSlug")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_slug: Option<String>,
    #[serde(rename = "rootCount")]
    #[serde(default)]
    pub root_count: i64,
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "spanCount")]
    #[serde(default)]
    pub span_count: i64,
    #[serde(rename = "traceId")]
    #[serde(default)]
    pub trace_id: String,
    #[serde(rename = "unknownOccurrenceCount")]
    #[serde(default)]
    pub unknown_occurrence_count: i64,
    #[serde(rename = "unknownSpanCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_span_count: Option<i64>,
    #[serde(rename = "wallDurationMs")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub wall_duration_ms: f64,
}

impl FindingOccurrence {
    pub fn builder() -> FindingOccurrenceBuilder {
        <FindingOccurrenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FindingOccurrenceBuilder {
    entry_name: Option<String>,
    entry_service: Option<String>,
    environment: Option<String>,
    error_count: Option<i64>,
    first_at: Option<String>,
    first_at_unix_nano: Option<UInt64String>,
    has_enriched_frames: Option<bool>,
    last_at_unix_nano: Option<UInt64String>,
    occurrences_in_trace: Option<i64>,
    release_slug: Option<String>,
    root_count: Option<i64>,
    session_id: Option<String>,
    span_count: Option<i64>,
    trace_id: Option<String>,
    unknown_occurrence_count: Option<i64>,
    unknown_span_count: Option<i64>,
    wall_duration_ms: Option<f64>,
}

impl FindingOccurrenceBuilder {
    pub fn entry_name(mut self, value: impl Into<String>) -> Self {
        self.entry_name = Some(value.into());
        self
    }

    pub fn entry_service(mut self, value: impl Into<String>) -> Self {
        self.entry_service = Some(value.into());
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn error_count(mut self, value: i64) -> Self {
        self.error_count = Some(value);
        self
    }

    pub fn first_at(mut self, value: impl Into<String>) -> Self {
        self.first_at = Some(value.into());
        self
    }

    pub fn first_at_unix_nano(mut self, value: UInt64String) -> Self {
        self.first_at_unix_nano = Some(value);
        self
    }

    pub fn has_enriched_frames(mut self, value: bool) -> Self {
        self.has_enriched_frames = Some(value);
        self
    }

    pub fn last_at_unix_nano(mut self, value: UInt64String) -> Self {
        self.last_at_unix_nano = Some(value);
        self
    }

    pub fn occurrences_in_trace(mut self, value: i64) -> Self {
        self.occurrences_in_trace = Some(value);
        self
    }

    pub fn release_slug(mut self, value: impl Into<String>) -> Self {
        self.release_slug = Some(value.into());
        self
    }

    pub fn root_count(mut self, value: i64) -> Self {
        self.root_count = Some(value);
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn span_count(mut self, value: i64) -> Self {
        self.span_count = Some(value);
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn unknown_occurrence_count(mut self, value: i64) -> Self {
        self.unknown_occurrence_count = Some(value);
        self
    }

    pub fn unknown_span_count(mut self, value: i64) -> Self {
        self.unknown_span_count = Some(value);
        self
    }

    pub fn wall_duration_ms(mut self, value: f64) -> Self {
        self.wall_duration_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FindingOccurrence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entry_name`](FindingOccurrenceBuilder::entry_name)
    /// - [`entry_service`](FindingOccurrenceBuilder::entry_service)
    /// - [`environment`](FindingOccurrenceBuilder::environment)
    /// - [`error_count`](FindingOccurrenceBuilder::error_count)
    /// - [`first_at`](FindingOccurrenceBuilder::first_at)
    /// - [`first_at_unix_nano`](FindingOccurrenceBuilder::first_at_unix_nano)
    /// - [`has_enriched_frames`](FindingOccurrenceBuilder::has_enriched_frames)
    /// - [`last_at_unix_nano`](FindingOccurrenceBuilder::last_at_unix_nano)
    /// - [`occurrences_in_trace`](FindingOccurrenceBuilder::occurrences_in_trace)
    /// - [`root_count`](FindingOccurrenceBuilder::root_count)
    /// - [`span_count`](FindingOccurrenceBuilder::span_count)
    /// - [`trace_id`](FindingOccurrenceBuilder::trace_id)
    /// - [`unknown_occurrence_count`](FindingOccurrenceBuilder::unknown_occurrence_count)
    /// - [`wall_duration_ms`](FindingOccurrenceBuilder::wall_duration_ms)
    pub fn build(self) -> Result<FindingOccurrence, BuildError> {
        Ok(FindingOccurrence {
            entry_name: self.entry_name.ok_or_else(|| BuildError::missing_field("entry_name"))?,
            entry_service: self.entry_service.ok_or_else(|| BuildError::missing_field("entry_service"))?,
            environment: self.environment.ok_or_else(|| BuildError::missing_field("environment"))?,
            error_count: self.error_count.ok_or_else(|| BuildError::missing_field("error_count"))?,
            first_at: self.first_at.ok_or_else(|| BuildError::missing_field("first_at"))?,
            first_at_unix_nano: self.first_at_unix_nano.ok_or_else(|| BuildError::missing_field("first_at_unix_nano"))?,
            has_enriched_frames: self.has_enriched_frames.ok_or_else(|| BuildError::missing_field("has_enriched_frames"))?,
            last_at_unix_nano: self.last_at_unix_nano.ok_or_else(|| BuildError::missing_field("last_at_unix_nano"))?,
            occurrences_in_trace: self.occurrences_in_trace.ok_or_else(|| BuildError::missing_field("occurrences_in_trace"))?,
            release_slug: self.release_slug,
            root_count: self.root_count.ok_or_else(|| BuildError::missing_field("root_count"))?,
            session_id: self.session_id,
            span_count: self.span_count.ok_or_else(|| BuildError::missing_field("span_count"))?,
            trace_id: self.trace_id.ok_or_else(|| BuildError::missing_field("trace_id"))?,
            unknown_occurrence_count: self.unknown_occurrence_count.ok_or_else(|| BuildError::missing_field("unknown_occurrence_count"))?,
            unknown_span_count: self.unknown_span_count,
            wall_duration_ms: self.wall_duration_ms.ok_or_else(|| BuildError::missing_field("wall_duration_ms"))?,
        })
    }
}
