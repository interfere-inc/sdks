use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SessionsClient {
    pub http_client: HttpClient,
}

impl SessionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Records a device heartbeat for a session.
    ///
    /// # Arguments
    ///
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
    ///         .sessions
    ///         .create_session(
    ///             &CreateSessionRequest {
    ///                 session_id: "01991111-1111-7111-8111-111111111111".to_string(),
    ///                 device_id: None,
    ///                 fp_hash: None,
    ///                 visitor_id: None,
    ///                 replay_trigger: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_session(
        &self,
        request: &CreateSessionRequest,
        options: Option<RequestOptions>,
    ) -> Result<SessionResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[
                    &["releaseTokenHeader"] as &[&str],
                    &["publicKeyHeader"] as &[&str],
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
                "v3/session",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Attaches an application user identity to a session.
    ///
    /// # Arguments
    ///
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
    ///         .sessions
    ///         .identify_session(
    ///             &IdentifySessionRequest {
    ///                 session_id: "01991111-1111-7111-8111-111111111111".to_string(),
    ///                 email: Some("customer@example.com".to_string()),
    ///                 identifier: "customer-123".to_string(),
    ///                 source: IdentifySource::Custom {
    ///                     data: CustomIdentifySource {
    ///                         name: "name".to_string(),
    ///                         ..Default::default()
    ///                     },
    ///                 },
    ///                 device_id: None,
    ///                 fp_hash: None,
    ///                 visitor_id: None,
    ///                 avatar: None,
    ///                 company: None,
    ///                 name: None,
    ///                 signature: None,
    ///                 traits: None,
    ///                 country: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn identify_session(
        &self,
        request: &IdentifySessionRequest,
        options: Option<RequestOptions>,
    ) -> Result<SessionResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[
                    &["releaseTokenHeader"] as &[&str],
                    &["publicKeyHeader"] as &[&str],
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
                "v3/session/identify",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns events keyed by session recording identity. Supply at most 10 identities, either as repeated query parameters or comma-separated values.
    ///
    /// # Arguments
    ///
    /// * `workspace` - Workspace URL slug, using lowercase letters, numbers, and hyphens.
    /// * `session_record_ids` - Repeat this parameter for multiple app (surface) UUID:session ID identities. CSV values are also accepted, with a total limit of 10 identities after splitting.
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
    ///         .sessions
    ///         .session_batch_events(
    ///             &"workspaceSlug".to_string(),
    ///             &"endUserId".to_string(),
    ///             &SessionBatchEventsQueryRequest {
    ///                 date_from: "date_from".to_string(),
    ///                 date_to: "date_to".to_string(),
    ///                 session_record_ids: vec![Some(
    ///                     "01991111-1111-7111-8111-111111111111:session-1".to_string(),
    ///                 )],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn session_batch_events(
        &self,
        workspace: &str,
        end_user_id: &str,
        request: &SessionBatchEventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SessionBatchEventsResponse, ApiError> {
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
                    "v3/workspaces/{}/consumers/{}/sessions/events",
                    workspace, end_user_id
                ),
                None,
                QueryBuilder::new()
                    .string("date_from", request.date_from.clone())
                    .string("date_to", request.date_to.clone())
                    .string_array("session_record_ids", request.session_record_ids.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns details of a session owned by the specified user.
    ///
    /// # Arguments
    ///
    /// * `workspace` - Workspace URL slug, using lowercase letters, numbers, and hyphens.
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
    ///         .sessions
    ///         .session_detail(
    ///             &"workspaceSlug".to_string(),
    ///             &"endUserId".to_string(),
    ///             &"sessionRecordId".to_string(),
    ///             &SessionDetailQueryRequest {
    ///                 date_from: "date_from".to_string(),
    ///                 date_to: "date_to".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn session_detail(
        &self,
        workspace: &str,
        end_user_id: &str,
        session_record_id: &str,
        request: &SessionDetailQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SessionDetailResponse, ApiError> {
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
                    "v3/workspaces/{}/consumers/{}/sessions/{}",
                    workspace, end_user_id, session_record_id
                ),
                None,
                QueryBuilder::new()
                    .string("date_from", request.date_from.clone())
                    .string("date_to", request.date_to.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Lists sessions in descending last-event order. For the next page, send both cursor fields from the final session: its lastEventAt formatted as a UTC date-time and its sessionId. Continue until a page contains fewer than the requested limit.
    ///
    /// # Arguments
    ///
    /// * `workspace` - Workspace URL slug, using lowercase letters, numbers, and hyphens.
    /// * `cursor_last_event_at` - Last session lastEventAt, converted from epoch milliseconds to UTC YYYY-MM-DD HH:mm:ss. Send together with cursor_session_id and cursor_device_id.
    /// * `cursor_session_id` - Last session sessionId from the previous page; breaks ties in descending last-event time.
    /// * `cursor_surface_id` - Last session surfaceId from the previous page.
    /// * `cursor_device_id` - Last session deviceId from the previous page.
    /// * `date_from` - First UTC day to include, formatted YYYY-MM-DD.
    /// * `date_to` - Last UTC day to include, formatted YYYY-MM-DD.
    /// * `limit` - Maximum sessions per page; defaults to 50.
    /// * `url` - Keeps sessions that visited a URL containing this text, case-insensitive.
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
    ///         .sessions
    ///         .list(
    ///             &"workspaceSlug".to_string(),
    ///             &"endUserId".to_string(),
    ///             &ListQueryRequest {
    ///                 date_from: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 date_to: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 cursor_last_event_at: None,
    ///                 cursor_session_id: None,
    ///                 cursor_surface_id: None,
    ///                 cursor_device_id: None,
    ///                 limit: None,
    ///                 surface_id: None,
    ///                 url: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        workspace: &str,
        end_user_id: &str,
        request: &ListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SessionsListResponse, ApiError> {
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
                    "v3/workspaces/{}/consumers/{}/sessions",
                    workspace, end_user_id
                ),
                None,
                QueryBuilder::new()
                    .string("cursor_last_event_at", request.cursor_last_event_at.clone())
                    .string("cursor_session_id", request.cursor_session_id.clone())
                    .serialize("cursor_surface_id", request.cursor_surface_id.clone())
                    .string("cursor_device_id", request.cursor_device_id.clone())
                    .date("date_from", request.date_from.clone())
                    .date("date_to", request.date_to.clone())
                    .int("limit", request.limit.clone())
                    .string("surface_id", request.surface_id.clone())
                    .string("url", request.url.clone())
                    .build(),
                options,
            )
            .await
    }
}
