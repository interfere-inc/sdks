use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct DrainClient {
    pub http_client: HttpClient,
}

impl DrainClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Accepts OTLP telemetry in protobuf or JSON. The logs signal also accepts Vercel NDJSON. HTTP 2xx means accepted for asynchronous processing, not that telemetry is already queryable in Basin. Delivery is at least once: retries can produce duplicates, and downstream consumers must deduplicate on packet_id. Clients should not retry a successful response or OTLP partial-success response; retry transport failures and retryable errors with backoff, honoring Retry-After. Preserve the original payload on retries. Acceptance is not a guarantee against telemetry loss if background processing fails. Authentication, payload validation and admission run before acknowledgment.
    ///
    /// # Arguments
    ///
    /// * `signal` - OTLP signal
    /// * `request` - OTLP JSON payload
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
    ///         .drain
    ///         .otlp_export(
    ///             &DrainOtlpExportRequestSignal::Traces,
    ///             &HashMap::from([("key".to_string(), serde_json::json!("value"))]),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn otlp_export(
        &self,
        signal: &DrainOtlpExportRequestSignal,
        request: &HashMap<String, serde_json::Value>,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[
                    &["cloudflareStreamId"] as &[&str],
                    &["workspaceTelemetryKey"] as &[&str],
                    &["workspaceTelemetryKeyHeader"] as &[&str],
                    &["releaseTokenHeader"] as &[&str],
                    &["publicKeyHeader"] as &[&str],
                    &["vercelDrainId"] as &[&str],
                ],
            )
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
                Method::POST,
                &format!("drain/v1/{}", signal),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
