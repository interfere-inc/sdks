use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod sessions;
pub use sessions::SessionsClient2;
pub struct ReplayClient {
    pub http_client: HttpClient,
    pub sessions: SessionsClient2,
}

impl ReplayClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            sessions: SessionsClient2::new(config.clone())?,
        })
    }

    /// Returns a page of replay chunk metadata for a recording.
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
    ///         .replay
    ///         .session_replay_index(
    ///             &"workspaceSlug".to_string(),
    ///             &"endUserId".to_string(),
    ///             &"sessionRecordId".to_string(),
    ///             &SessionReplayIndexQueryRequest {
    ///                 date_from: "date_from".to_string(),
    ///                 date_to: "date_to".to_string(),
    ///                 after: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn session_replay_index(
        &self,
        workspace: &str,
        end_user_id: &str,
        session_record_id: &str,
        request: &SessionReplayIndexQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SessionReplayIndexResponse, ApiError> {
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
                &format!(
                    "v3/workspaces/{}/consumers/{}/sessions/{}/replay/index",
                    workspace, end_user_id, session_record_id
                ),
                None,
                QueryBuilder::new()
                    .string("date_from", request.date_from.clone())
                    .string("date_to", request.date_to.clone())
                    .int("after", request.after.clone())
                    .build(),
                options,
            )
            .await
    }
}
