use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TracesClient {
    pub http_client: HttpClient,
}

impl TracesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Reads nearby activity correlated to a finding occurrence by session and page view. These records can belong to separate execution traces.
    ///
    /// # Arguments
    ///
    /// * `finding_id` - Finding whose traces are requested
    /// * `trace_id` - W3C trace id
    /// * `occurred_at` - occurredAt is required: pass the trace's firstAtUnixNano from GET /v3/findings/{findingId}/traces
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use interfere_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = InterfereClient::new(config).expect("Failed to build client");
    ///     client
    ///         .traces
    ///         .trace_activity(
    ///             &"findingId".to_string(),
    ///             &"4bf92f3577b34da6a3ce929d0e0e4736".to_string(),
    ///             &TraceActivityQueryRequest {
    ///                 occurred_at: TraceActivityRequestOccurredAt::String("occurredAt".to_string()),
    ///                 limit: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn trace_activity(
        &self,
        finding_id: &str,
        trace_id: &str,
        request: &TraceActivityQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TraceActivity, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["clerkSession"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v3/findings/{}/traces/{}/activity", finding_id, trace_id),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .serialize("occurredAt", Some(request.occurred_at.clone()))
                    .build(),
                options,
            )
            .await
    }

    /// Lists the traces where a finding occurred, newest first.
    ///
    /// # Arguments
    ///
    /// * `finding_id` - Finding whose traces are requested
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use interfere_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = InterfereClient::new(config).expect("Failed to build client");
    ///     client
    ///         .traces
    ///         .finding_occurrences(
    ///             &"findingId".to_string(),
    ///             &FindingOccurrencesQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn finding_occurrences(
        &self,
        finding_id: &str,
        request: &FindingOccurrencesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<FindingOccurrences, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["clerkSession"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v3/findings/{}/traces", finding_id),
                None,
                QueryBuilder::new()
                    .string("cursor", request.cursor.clone())
                    .string("environment", request.environment.clone())
                    .int("limit", request.limit.clone())
                    .string("releaseSlug", request.release_slug.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Reads every retained span in one trace of a finding.
    ///
    /// # Arguments
    ///
    /// * `finding_id` - Finding whose traces are requested
    /// * `trace_id` - W3C trace id
    /// * `occurred_at` - occurredAt is required: pass the trace's firstAtUnixNano from GET /v3/findings/{findingId}/traces
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use interfere_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = InterfereClient::new(config).expect("Failed to build client");
    ///     client
    ///         .traces
    ///         .trace_detail(
    ///             &"findingId".to_string(),
    ///             &"4bf92f3577b34da6a3ce929d0e0e4736".to_string(),
    ///             &TraceDetailQueryRequest {
    ///                 occurred_at: TraceDetailRequestOccurredAt::String("occurredAt".to_string()),
    ///                 limit: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn trace_detail(
        &self,
        finding_id: &str,
        trace_id: &str,
        request: &TraceDetailQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TraceDetail, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["clerkSession"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v3/findings/{}/traces/{}", finding_id, trace_id),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .serialize("occurredAt", Some(request.occurred_at.clone()))
                    .build(),
                options,
            )
            .await
    }

    /// Reads the log records emitted within one trace of a finding, oldest first. A generic exception message often hides the condition that a log line in the same trace names.
    ///
    /// # Arguments
    ///
    /// * `finding_id` - Finding whose traces are requested
    /// * `trace_id` - W3C trace id
    /// * `occurred_at` - occurredAt is required: pass the trace's firstAtUnixNano from GET /v3/findings/{findingId}/traces
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use interfere_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = InterfereClient::new(config).expect("Failed to build client");
    ///     client
    ///         .traces
    ///         .trace_logs(
    ///             &"findingId".to_string(),
    ///             &"4bf92f3577b34da6a3ce929d0e0e4736".to_string(),
    ///             &TraceLogsQueryRequest {
    ///                 occurred_at: TraceLogsRequestOccurredAt::String("occurredAt".to_string()),
    ///                 limit: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn trace_logs(
        &self,
        finding_id: &str,
        trace_id: &str,
        request: &TraceLogsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TraceLogs, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["clerkSession"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v3/findings/{}/traces/{}/logs", finding_id, trace_id),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .serialize("occurredAt", Some(request.occurred_at.clone()))
                    .build(),
                options,
            )
            .await
    }

    /// Reads one span exactly as its instrumentation reported it, including the raw OTel attribute blob the trace read omits.
    ///
    /// # Arguments
    ///
    /// * `finding_id` - Finding whose traces are requested
    /// * `trace_id` - W3C trace id
    /// * `span_id` - W3C span id
    /// * `ended_at_unix_nano` - Optional endedAtUnixNano from GET /v3/findings/{findingId}/traces/{traceId}; narrows the read to the trace's extent within the window days
    /// * `started_at_unix_nano` - Optional startedAtUnixNano from GET /v3/findings/{findingId}/traces/{traceId}; narrows the read to the trace's extent within the window days
    /// * `window_end` - windowEnd is required: pass windowEnd from GET /v3/findings/{findingId}/traces/{traceId}
    /// * `window_start` - windowStart is required: pass windowStart from GET /v3/findings/{findingId}/traces/{traceId}
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use interfere_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = InterfereClient::new(config).expect("Failed to build client");
    ///     client
    ///         .traces
    ///         .span_detail(
    ///             &"findingId".to_string(),
    ///             &"4bf92f3577b34da6a3ce929d0e0e4736".to_string(),
    ///             &"00f067aa0ba902b7".to_string(),
    ///             &SpanDetailQueryRequest {
    ///                 window_end: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 window_start: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 ended_at_unix_nano: None,
    ///                 started_at_unix_nano: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn span_detail(
        &self,
        finding_id: &str,
        trace_id: &str,
        span_id: &str,
        request: &SpanDetailQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SpanDetail, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["clerkSession"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v3/findings/{}/traces/{}/spans/{}",
                    finding_id, trace_id, span_id
                ),
                None,
                QueryBuilder::new()
                    .string("endedAtUnixNano", request.ended_at_unix_nano.clone())
                    .string("startedAtUnixNano", request.started_at_unix_nano.clone())
                    .date("windowEnd", request.window_end.clone())
                    .date("windowStart", request.window_start.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Reads exception frames for the selected span within its trace window.
    ///
    /// # Arguments
    ///
    /// * `finding_id` - Finding whose traces are requested
    /// * `trace_id` - W3C trace id
    /// * `span_id` - W3C span id
    /// * `ended_at_unix_nano` - Optional endedAtUnixNano from GET /v3/findings/{findingId}/traces/{traceId}; narrows the read to the trace's extent within the window days
    /// * `started_at_unix_nano` - Optional startedAtUnixNano from GET /v3/findings/{findingId}/traces/{traceId}; narrows the read to the trace's extent within the window days
    /// * `window_end` - windowEnd is required: pass windowEnd from GET /v3/findings/{findingId}/traces/{traceId}
    /// * `window_start` - windowStart is required: pass windowStart from GET /v3/findings/{findingId}/traces/{traceId}
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use interfere_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = InterfereClient::new(config).expect("Failed to build client");
    ///     client
    ///         .traces
    ///         .span_stack(
    ///             &"findingId".to_string(),
    ///             &"4bf92f3577b34da6a3ce929d0e0e4736".to_string(),
    ///             &"00f067aa0ba902b7".to_string(),
    ///             &SpanStackQueryRequest {
    ///                 window_end: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 window_start: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 ended_at_unix_nano: None,
    ///                 started_at_unix_nano: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn span_stack(
        &self,
        finding_id: &str,
        trace_id: &str,
        span_id: &str,
        request: &SpanStackQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SpanStack, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["clerkSession"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v3/findings/{}/traces/{}/spans/{}/stack",
                    finding_id, trace_id, span_id
                ),
                None,
                QueryBuilder::new()
                    .string("endedAtUnixNano", request.ended_at_unix_nano.clone())
                    .string("startedAtUnixNano", request.started_at_unix_nano.clone())
                    .date("windowEnd", request.window_end.clone())
                    .date("windowStart", request.window_start.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Reads the recent durations of one span identity, oldest first, with the median over the sampled window. Identity is the (serviceName, name) pair the span row already carries; kind narrows the read to one granule range.
    ///
    /// # Arguments
    ///
    /// * `finding_id` - Finding whose traces are requested
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use interfere_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = InterfereClient::new(config).expect("Failed to build client");
    ///     client
    ///         .traces
    ///         .span_duration_history(
    ///             &"findingId".to_string(),
    ///             &SpanDurationHistoryQueryRequest {
    ///                 kind: "kind".to_string(),
    ///                 name: "name".to_string(),
    ///                 service_name: "serviceName".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn span_duration_history(
        &self,
        finding_id: &str,
        request: &SpanDurationHistoryQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SpanDurationHistory, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["clerkSession"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v3/findings/{}/span-duration-history", finding_id),
                None,
                QueryBuilder::new()
                    .string("kind", request.kind.clone())
                    .string("name", request.name.clone())
                    .string("serviceName", request.service_name.clone())
                    .build(),
                options,
            )
            .await
    }
}
