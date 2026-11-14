use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct TimelineClient {
    pub http_client: HttpClient,
}

impl TimelineClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Post a comment to a problem timeline. Resolve problemId first; this publishes content to other workspace members. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.
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
    ///         .timeline
    ///         .create_comment(
    ///             &"workspaceSlug".to_string(),
    ///             &CreateCommentTimelineRequest {
    ///                 args: CreateCommentTimelineRequestArgs {
    ///                     id: "id".to_string(),
    ///                     problem_id: "problemId".to_string(),
    ///                     parent_id: None,
    ///                     doc: TiptapDocument {
    ///                         content: vec![TiptapDocumentContentItem::TiptapDocumentContentItemZero(
    ///                             TiptapDocumentContentItemZero {
    ///                                 content: None,
    ///                                 r#type: TiptapDocumentContentItemZeroType::Paragraph,
    ///                             },
    ///                         )],
    ///                         r#type: TiptapDocumentType::Doc,
    ///                     },
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_comment(
        &self,
        workspace: &str,
        request: &CreateCommentTimelineRequest,
        options: Option<RequestOptions>,
    ) -> Result<MutationTimelineEventsCreateCommentResponse, ApiError> {
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
                    "v3/workspaces/{}/actions/timelineEvents.createComment",
                    workspace
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Respond to a problem action request. Inspect problems.actionRequests and the problem timeline before approving or rejecting the proposed action. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.
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
    ///         .timeline
    ///         .respond_to_action(
    ///             &"workspaceSlug".to_string(),
    ///             &RespondToActionTimelineRequest {
    ///                 args: RespondToActionTimelineRequestArgs {
    ///                     id: "id".to_string(),
    ///                     problem_id: "problemId".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn respond_to_action(
        &self,
        workspace: &str,
        request: &RespondToActionTimelineRequest,
        options: Option<RequestOptions>,
    ) -> Result<MutationTimelineEventsRespondToActionResponse, ApiError> {
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
                    "v3/workspaces/{}/actions/timelineEvents.respondToAction",
                    workspace
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Remove a timeline comment using its event ID from problems.get. Verify authorship and target before acting. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.
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
    ///         .timeline
    ///         .soft_delete_comment(
    ///             &"workspaceSlug".to_string(),
    ///             &SoftDeleteCommentTimelineRequest {
    ///                 args: SoftDeleteCommentTimelineRequestArgs {
    ///                     id: "id".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn soft_delete_comment(
        &self,
        workspace: &str,
        request: &SoftDeleteCommentTimelineRequest,
        options: Option<RequestOptions>,
    ) -> Result<MutationTimelineEventsSoftDeleteCommentResponse, ApiError> {
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
                    "v3/workspaces/{}/actions/timelineEvents.softDeleteComment",
                    workspace
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Toggle a reaction on a timeline event. Repeating the same request can reverse the intended reaction. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.
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
    ///         .timeline
    ///         .toggle_reaction(
    ///             &"workspaceSlug".to_string(),
    ///             &ToggleReactionTimelineRequest {
    ///                 args: ToggleReactionTimelineRequestArgs {
    ///                     reaction_id: "reactionId".to_string(),
    ///                     timeline_event_id: "timelineEventId".to_string(),
    ///                     emoji: "emoji".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn toggle_reaction(
        &self,
        workspace: &str,
        request: &ToggleReactionTimelineRequest,
        options: Option<RequestOptions>,
    ) -> Result<MutationTimelineEventsToggleReactionResponse, ApiError> {
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
                    "v3/workspaces/{}/actions/timelineEvents.toggleReaction",
                    workspace
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Edit an existing problem timeline comment. Read the current text and event ID from problems.get first. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.
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
    ///         .timeline
    ///         .update_comment(
    ///             &"workspaceSlug".to_string(),
    ///             &UpdateCommentTimelineRequest {
    ///                 args: UpdateCommentTimelineRequestArgs {
    ///                     id: "id".to_string(),
    ///                     doc: TiptapDocument {
    ///                         content: vec![TiptapDocumentContentItem::TiptapDocumentContentItemZero(
    ///                             TiptapDocumentContentItemZero {
    ///                                 content: None,
    ///                                 r#type: TiptapDocumentContentItemZeroType::Paragraph,
    ///                             },
    ///                         )],
    ///                         r#type: TiptapDocumentType::Doc,
    ///                     },
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_comment(
        &self,
        workspace: &str,
        request: &UpdateCommentTimelineRequest,
        options: Option<RequestOptions>,
    ) -> Result<MutationTimelineEventsUpdateCommentResponse, ApiError> {
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
                    "v3/workspaces/{}/actions/timelineEvents.updateComment",
                    workspace
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
