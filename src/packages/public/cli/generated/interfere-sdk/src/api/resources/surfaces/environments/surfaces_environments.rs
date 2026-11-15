use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct EnvironmentsClient {
    pub http_client: HttpClient,
}

impl EnvironmentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List app (surface) environments used for inbox filtering. Use returned identifiers when selecting an environment. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.
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
    ///         .surfaces
    ///         .environments
    ///         .for_inbox(
    ///             &"workspaceSlug".to_string(),
    ///             &ForInboxEnvironmentsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn for_inbox(
        &self,
        workspace: &str,
        request: &ForInboxEnvironmentsRequest,
        options: Option<RequestOptions>,
    ) -> Result<QuerySurfaceEnvironmentsForInboxResponse, ApiError> {
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
                    "v3/workspaces/{}/queries/surfaceEnvironments.forInbox",
                    workspace
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List an app's (surface's) environments to resolve environment identifiers before filtering problems or activity. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.
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
    ///         .surfaces
    ///         .environments
    ///         .list_for_surface(
    ///             &"workspaceSlug".to_string(),
    ///             &ListForSurfaceEnvironmentsRequest {
    ///                 args: ListForSurfaceEnvironmentsRequestArgs {
    ///                     surface_id: "surfaceId".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_for_surface(
        &self,
        workspace: &str,
        request: &ListForSurfaceEnvironmentsRequest,
        options: Option<RequestOptions>,
    ) -> Result<QuerySurfaceEnvironmentsForSurfaceResponse, ApiError> {
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
                    "v3/workspaces/{}/queries/surfaceEnvironments.forSurface",
                    workspace
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete an app (surface) environment identified from surfaces.environments.listForSurface. Verify the intended app and environment. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.
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
    ///         .surfaces
    ///         .environments
    ///         .delete(
    ///             &"workspaceSlug".to_string(),
    ///             &DeleteEnvironmentsRequest {
    ///                 args: DeleteEnvironmentsRequestArgs {
    ///                     id: "id".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        workspace: &str,
        request: &DeleteEnvironmentsRequest,
        options: Option<RequestOptions>,
    ) -> Result<MutationSurfaceEnvironmentsDeleteResponse, ApiError> {
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
                Method::POST,
                &format!(
                    "v3/workspaces/{}/actions/surfaceEnvironments.delete",
                    workspace
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Mark an app (surface) environment as production or not. Problems seen only in non-production environments skip workspace Slack, personal Slack and desktop push notifications. Identify the environment from surfaces.environments.listForSurface. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.
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
    ///         .surfaces
    ///         .environments
    ///         .set_production(
    ///             &"workspaceSlug".to_string(),
    ///             &SetProductionEnvironmentsRequest {
    ///                 args: SetProductionEnvironmentsRequestArgs {
    ///                     id: "id".to_string(),
    ///                     is_production: true,
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn set_production(
        &self,
        workspace: &str,
        request: &SetProductionEnvironmentsRequest,
        options: Option<RequestOptions>,
    ) -> Result<MutationSurfaceEnvironmentsSetProductionResponse, ApiError> {
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
                Method::POST,
                &format!(
                    "v3/workspaces/{}/actions/surfaceEnvironments.setProduction",
                    workspace
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Turn Problem reporting for an app (surface) environment on or off. Off stops new Problems from that environment; existing Problems stay. Identify the environment from surfaces.environments.listForSurface. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.
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
    ///         .surfaces
    ///         .environments
    ///         .set_reportable(
    ///             &"workspaceSlug".to_string(),
    ///             &SetReportableEnvironmentsRequest {
    ///                 args: SetReportableEnvironmentsRequestArgs {
    ///                     id: "id".to_string(),
    ///                     reportable: true,
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn set_reportable(
        &self,
        workspace: &str,
        request: &SetReportableEnvironmentsRequest,
        options: Option<RequestOptions>,
    ) -> Result<MutationSurfaceEnvironmentsSetReportableResponse, ApiError> {
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
                Method::POST,
                &format!(
                    "v3/workspaces/{}/actions/surfaceEnvironments.setReportable",
                    workspace
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
