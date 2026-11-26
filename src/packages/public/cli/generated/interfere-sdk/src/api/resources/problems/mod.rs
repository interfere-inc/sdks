use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub mod timeline;
pub use timeline::TimelineClient;
pub struct ProblemsClient {
    pub http_client: HttpClient,
    pub timeline: TimelineClient,
}

impl ProblemsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            timeline: TimelineClient::new(config.clone())?,
        })
    }

    /// Get a problem whose category, priority, and assignee can be edited after investigation confirmation. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.
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
    ///         .problems
    ///         .editable_properties(
    ///             &"workspaceSlug".to_string(),
    ///             &EditablePropertiesProblemsRequest {
    ///                 args: EditablePropertiesProblemsRequestArgs {
    ///                     problem_id: "problemId".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn editable_properties(
        &self,
        workspace: &str,
        request: &EditablePropertiesProblemsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Option<QueryProblemsEditablePropertiesResponse>, ApiError> {
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
                    "v3/workspaces/{}/queries/problems.editableProperties",
                    workspace
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List outstanding problem action-request timeline records for inspection. Reading does not approve an action. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.
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
    ///         .problems
    ///         .action_requests(
    ///             &"workspaceSlug".to_string(),
    ///             &ActionRequestsProblemsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn action_requests(
        &self,
        workspace: &str,
        request: &ActionRequestsProblemsRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryProblemsActionRequestsResponse, ApiError> {
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
                    "v3/workspaces/{}/queries/problems.actionRequests",
                    workspace
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read archived investigation accounts preceding a Problem, scoped to the current Workspace. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.
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
    ///         .problems
    ///         .previous_accounts(
    ///             &"workspaceSlug".to_string(),
    ///             &PreviousAccountsProblemsRequest {
    ///                 args: PreviousAccountsProblemsRequestArgs {
    ///                     problem_id: "problemId".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn previous_accounts(
        &self,
        workspace: &str,
        request: &PreviousAccountsProblemsRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryProblemsPreviousAccountsResponse, ApiError> {
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
                    "v3/workspaces/{}/queries/problems.previousAccounts",
                    workspace
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read a visible problem with its app (surface), assignee and timeline, including comments and references. Obtain problemId from problems.list before investigating details. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.
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
    ///         .problems
    ///         .get(
    ///             &"workspaceSlug".to_string(),
    ///             &GetProblemsRequest {
    ///                 args: GetProblemsRequestArgs {
    ///                     problem_id: "problemId".to_string(),
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
        request: &GetProblemsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Option<QueryProblemsGetByIdResponse>, ApiError> {
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
                &format!("v3/workspaces/{}/queries/problems.getById", workspace),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List visible problems for a board, optionally filtered by environmentNames from environments.list. The app (surface)-scoped environments filter is deprecated. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.
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
    ///         .problems
    ///         .kanban(
    ///             &"workspaceSlug".to_string(),
    ///             &KanbanProblemsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn kanban(
        &self,
        workspace: &str,
        request: &KanbanProblemsRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryProblemsKanbanResponse, ApiError> {
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
                &format!("v3/workspaces/{}/queries/problems.kanban", workspace),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List visible, non-deleted problems ordered by lastSeenAt descending, including assignees. Use IDs from this result for problem details, findings and affected-user endpoints. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.
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
    ///         .problems
    ///         .list(
    ///             &"workspaceSlug".to_string(),
    ///             &ListProblemsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        workspace: &str,
        request: &ListProblemsRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryProblemsListResponse, ApiError> {
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
                &format!("v3/workspaces/{}/queries/problems.list", workspace),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List visible, non-deleted problems with each problem's newest customer-visible timeline event (detection, status change, observation, regression, action request, comment, root cause or fix). Order by the later of createdAt and that event's createdAt to rank problems by recent activity. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.
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
    ///         .problems
    ///         .navigable(
    ///             &"workspaceSlug".to_string(),
    ///             &NavigableProblemsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn navigable(
        &self,
        workspace: &str,
        request: &NavigableProblemsRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryProblemsNavigableResponse, ApiError> {
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
                &format!("v3/workspaces/{}/queries/problems.navigable", workspace),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Dismiss a problem with a reason: noise, notAMalfunction or wontFix, or duplicateOf with the problemId of the Problem that already tracks the defect (resolve it with problems.list first). Inspect its current evidence and state before applying the requested dismissal. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.
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
    ///         .problems
    ///         .dismiss(
    ///             &"workspaceSlug".to_string(),
    ///             &DismissProblemsRequest {
    ///                 args: DismissProblemsRequestArgs {
    ///                     problem_id: "problemId".to_string(),
    ///                     timeline_event_id: "timelineEventId".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn dismiss(
        &self,
        workspace: &str,
        request: &DismissProblemsRequest,
        options: Option<RequestOptions>,
    ) -> Result<MutationProblemsDismissResponse, ApiError> {
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
                &format!("v3/workspaces/{}/actions/problems.dismiss", workspace),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Reopen a dismissed or not-a-problem Problem as allowed by the lifecycle rules. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.
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
    ///         .problems
    ///         .restore(
    ///             &"workspaceSlug".to_string(),
    ///             &RestoreProblemsRequest {
    ///                 args: RestoreProblemsRequestArgs {
    ///                     problem_id: "problemId".to_string(),
    ///                     timeline_event_id: "timelineEventId".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn restore(
        &self,
        workspace: &str,
        request: &RestoreProblemsRequest,
        options: Option<RequestOptions>,
    ) -> Result<MutationProblemsRestoreResponse, ApiError> {
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
                &format!("v3/workspaces/{}/actions/problems.restore", workspace),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Change a confirmed problem's assignee. Resolve the problem and workspace member IDs before writing. Unconfirmed problems are left unchanged. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.
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
    ///         .problems
    ///         .update_assignee(
    ///             &"workspaceSlug".to_string(),
    ///             &UpdateAssigneeProblemsRequest {
    ///                 args: UpdateAssigneeProblemsRequestArgs {
    ///                     problem_id: "problemId".to_string(),
    ///                     assignee_user_id: None,
    ///                     timeline_event_id: "timelineEventId".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_assignee(
        &self,
        workspace: &str,
        request: &UpdateAssigneeProblemsRequest,
        options: Option<RequestOptions>,
    ) -> Result<MutationProblemsUpdateAssigneeResponse, ApiError> {
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
                    "v3/workspaces/{}/actions/problems.updateAssignee",
                    workspace
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Change a confirmed problem's category using a value allowed by the input schema. Unconfirmed problems are left unchanged. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.
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
    ///         .problems
    ///         .update_category(
    ///             &"workspaceSlug".to_string(),
    ///             &UpdateCategoryProblemsRequest {
    ///                 args: UpdateCategoryProblemsRequestArgs {
    ///                     problem_id: "problemId".to_string(),
    ///                     category: None,
    ///                     timeline_event_id: "timelineEventId".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_category(
        &self,
        workspace: &str,
        request: &UpdateCategoryProblemsRequest,
        options: Option<RequestOptions>,
    ) -> Result<MutationProblemsUpdateCategoryResponse, ApiError> {
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
                    "v3/workspaces/{}/actions/problems.updateCategory",
                    workspace
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Update a problem's editable details. Read problems.get first and preserve fields the user did not ask to change. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.
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
    ///         .problems
    ///         .update_details(
    ///             &"workspaceSlug".to_string(),
    ///             &UpdateDetailsProblemsRequest {
    ///                 args: UpdateDetailsProblemsRequestArgs {
    ///                     problem_id: "problemId".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_details(
        &self,
        workspace: &str,
        request: &UpdateDetailsProblemsRequest,
        options: Option<RequestOptions>,
    ) -> Result<MutationProblemsUpdateDetailsResponse, ApiError> {
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
                &format!("v3/workspaces/{}/actions/problems.updateDetails", workspace),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Change a confirmed problem's priority using a value allowed by the input schema. Unconfirmed problems are left unchanged. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.
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
    ///         .problems
    ///         .update_priority(
    ///             &"workspaceSlug".to_string(),
    ///             &UpdatePriorityProblemsRequest {
    ///                 args: UpdatePriorityProblemsRequestArgs {
    ///                     problem_id: "problemId".to_string(),
    ///                     priority: None,
    ///                     timeline_event_id: "timelineEventId".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_priority(
        &self,
        workspace: &str,
        request: &UpdatePriorityProblemsRequest,
        options: Option<RequestOptions>,
    ) -> Result<MutationProblemsUpdatePriorityResponse, ApiError> {
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
                    "v3/workspaces/{}/actions/problems.updatePriority",
                    workspace
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns the website services whose spans share a trace with the problem's newest occurrences in its last seven days. availability is unavailable when the problem has no owner to sample.
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
    ///         .problems
    ///         .problem_affected_services(&"workspaceSlug".to_string(), &"problemId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn problem_affected_services(
        &self,
        workspace: &str,
        problem_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ProblemAffectedServicesResponse, ApiError> {
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
                    "v3/workspaces/{}/problems/{}/affectedServices",
                    workspace, problem_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns users affected by a problem within the requested time range.
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
    ///         .problems
    ///         .affected_by_problem(&"workspaceSlug".to_string(), &"problemId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn affected_by_problem(
        &self,
        workspace: &str,
        problem_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AffectedByProblemResponse, ApiError> {
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
                    "v3/workspaces/{}/problems/{}/affectedUsers",
                    workspace, problem_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns affected-user counts grouped into time buckets for a problem.
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
    ///         .problems
    ///         .affected_by_problem_timeseries(
    ///             &"workspaceSlug".to_string(),
    ///             &"problemId".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn affected_by_problem_timeseries(
        &self,
        workspace: &str,
        problem_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AffectedByProblemTimeseriesResponse, ApiError> {
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
                    "v3/workspaces/{}/problems/{}/affectedUsers/timeseries",
                    workspace, problem_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns the findings grouped into a problem with their fingerprints, representative occurrences with their exception messages, and the investigator's Observations with the evidence each one cited, such as source files and lines, Facts, spans and logs.
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
    ///         .problems
    ///         .problem_evidence(&"workspaceSlug".to_string(), &"problemId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn problem_evidence(
        &self,
        workspace: &str,
        problem_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ProblemEvidenceResponse, ApiError> {
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
                    "v3/workspaces/{}/problems/{}/evidence",
                    workspace, problem_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns the repository linked to the failing code, the commit of the newest captured occurrence, and the builds that served after it, so a fix can be checked against the build that first contains it.
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
    ///         .problems
    ///         .problem_source(&"workspaceSlug".to_string(), &"problemId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn problem_source(
        &self,
        workspace: &str,
        problem_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ProblemSourceResponse, ApiError> {
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
                &format!("v3/workspaces/{}/problems/{}/source", workspace, problem_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns the problem this one was merged into, problems merged into it, server problems whose errors ran in the same failing requests, and earlier problems with a recorded root cause in the same code area.
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
    ///         .problems
    ///         .problem_related(&"workspaceSlug".to_string(), &"problemId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn problem_related(
        &self,
        workspace: &str,
        problem_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ProblemRelatedResponse, ApiError> {
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
                    "v3/workspaces/{}/problems/{}/related",
                    workspace, problem_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns the problem's occurrences and recorded affected users over the last 7 days, with its first and last seen times.
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
    ///         .problems
    ///         .problem_impact(&"workspaceSlug".to_string(), &"problemId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn problem_impact(
        &self,
        workspace: &str,
        problem_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ProblemImpactResponse, ApiError> {
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
                &format!("v3/workspaces/{}/problems/{}/impact", workspace, problem_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns daily occurrences and recorded affected-user counts for the last 30 UTC days.
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
    ///         .problems
    ///         .problem_timeseries(&"workspaceSlug".to_string(), &"problemId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn problem_timeseries(
        &self,
        workspace: &str,
        problem_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ProblemTimeseriesResponse, ApiError> {
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
                    "v3/workspaces/{}/problems/{}/timeseries",
                    workspace, problem_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns sessions associated with a problem in the workspace.
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
    ///         .problems
    ///         .problem_sessions(&"workspaceSlug".to_string(), &"problemId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn problem_sessions(
        &self,
        workspace: &str,
        problem_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ProblemSessionsResponse, ApiError> {
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
                    "v3/workspaces/{}/problems/{}/sessions",
                    workspace, problem_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Records the root cause a workspace member established. It supersedes the investigation's root cause, which is kept as the superseded version. Name code by repository, path and line range; never paste code. With completeEnding, an investigation that failed or is waiting for an answer completes with this root cause and its open questions are withdrawn. Refused while the investigation is running. Retry with the same request ID: a retry finishes a completion the first attempt started.
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
    ///         .problems
    ///         .correct_root_cause(
    ///             &"workspaceSlug".to_string(),
    ///             &"problemId".to_string(),
    ///             &CorrectRootCauseRequest {
    ///                 request_id: "requestId".to_string(),
    ///                 cause: CorrectRootCauseRequestCause {
    ///                     summary: "summary".to_string(),
    ///                     cause_class: CorrectRootCauseRequestCauseCauseClass::Code,
    ///                     locations: None,
    ///                 },
    ///                 reason: CorrectRootCauseRequestReason::WrongMechanism,
    ///                 complete_ending: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn correct_root_cause(
        &self,
        workspace: &str,
        problem_id: &str,
        request: &CorrectRootCauseRequest,
        options: Option<RequestOptions>,
    ) -> Result<CorrectRootCauseResponse, ApiError> {
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
                    "v3/workspaces/{}/problems/{}/root-cause/correction",
                    workspace, problem_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Records the fix a workspace member established. It supersedes the investigation's fix, which is kept as the superseded version. Name code by repository, path and line range; never paste code. Refused while the investigation is running. Retry with the same request ID.
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
    ///         .problems
    ///         .correct_fix(
    ///             &"workspaceSlug".to_string(),
    ///             &"problemId".to_string(),
    ///             &CorrectFixRequest {
    ///                 request_id: "requestId".to_string(),
    ///                 summary: "summary".to_string(),
    ///                 reason: CorrectFixRequestReason::WrongMechanism,
    ///                 locations: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn correct_fix(
        &self,
        workspace: &str,
        problem_id: &str,
        request: &CorrectFixRequest,
        options: Option<RequestOptions>,
    ) -> Result<CorrectRootCauseResponse, ApiError> {
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
                    "v3/workspaces/{}/problems/{}/fix/correction",
                    workspace, problem_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Merges this problem into the open problem it duplicates. Its findings and occurrences move to that problem, and both problems record the merge. Refused, with a reason, when the problems cannot merge, for example when the keeper is closed or they belong to different services. Retry with the same request ID.
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
    ///         .problems
    ///         .mark_problem_duplicate(
    ///             &"workspaceSlug".to_string(),
    ///             &"problemId".to_string(),
    ///             &MarkProblemDuplicateRequest {
    ///                 request_id: "requestId".to_string(),
    ///                 keeper_problem_id: "keeperProblemId".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn mark_problem_duplicate(
        &self,
        workspace: &str,
        problem_id: &str,
        request: &MarkProblemDuplicateRequest,
        options: Option<RequestOptions>,
    ) -> Result<MarkProblemDuplicateResponse, ApiError> {
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
                    "v3/workspaces/{}/problems/{}/duplicate",
                    workspace, problem_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
