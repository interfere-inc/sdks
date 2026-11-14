use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AuditClient {
    pub http_client: HttpClient,
}

impl AuditClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List workspace audit events to investigate who changed a resource. Use the schema's filters and pagination to narrow the time range and target. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.
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
    ///         .audit
    ///         .list(
    ///             &"workspaceSlug".to_string(),
    ///             &ListAuditRequest {
    ///                 args: ListAuditRequestArgs {
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        workspace: &str,
        request: &ListAuditRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryWorkspaceAuditEventsListResponse, ApiError> {
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
                Method::POST,
                &format!(
                    "v3/workspaces/{}/queries/workspaceAuditEvents.list",
                    workspace
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read one workspace audit event by its event ID from audit.list. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.
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
    ///         .audit
    ///         .get(
    ///             &"workspaceSlug".to_string(),
    ///             &GetAuditRequest {
    ///                 args: GetAuditRequestArgs {
    ///                     id: "id".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        workspace: &str,
        request: &GetAuditRequest,
        options: Option<RequestOptions>,
    ) -> Result<Option<QueryWorkspaceAuditEventsByIdResponse>, ApiError> {
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
                Method::POST,
                &format!(
                    "v3/workspaces/{}/queries/workspaceAuditEvents.byId",
                    workspace
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a capped count of matching audit events for workspace owners and administrators.
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
    ///         .audit
    ///         .count(
    ///             &"workspaceSlug".to_string(),
    ///             &CountQueryRequest {
    ///                 search: None,
    ///                 search_actions: vec![],
    ///                 actions: vec![],
    ///                 actors: vec![],
    ///                 since: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn count(
        &self,
        workspace: &str,
        request: &CountQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AuditCountResponse, ApiError> {
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
                &format!("v3/workspaces/{}/audit/count", workspace),
                None,
                QueryBuilder::new()
                    .string("search", request.search.clone())
                    .string_array("searchActions", request.search_actions.clone())
                    .string_array("actions", request.actions.clone())
                    .string_array("actors", request.actors.clone())
                    .float("since", request.since.clone())
                    .build(),
                options,
            )
            .await
    }
}
