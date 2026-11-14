use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod devices;
pub use devices::DevicesClient;
pub struct ConsumersClient {
    pub http_client: HttpClient,
    pub devices: DevicesClient,
}

impl ConsumersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            devices: DevicesClient::new(config.clone())?,
        })
    }

    /// Read a tracked application user by internal user ID. This is distinct from an Interfere member or external identity. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.
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
    ///         .consumers
    ///         .get(
    ///             &"workspaceSlug".to_string(),
    ///             &GetConsumersRequest {
    ///                 args: GetConsumersRequestArgs {
    ///                     end_user_id: "endUserId".to_string(),
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
        request: &GetConsumersRequest,
        options: Option<RequestOptions>,
    ) -> Result<Option<QueryConsumersGetByIdResponse>, ApiError> {
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
                &format!("v3/workspaces/{}/queries/consumers.getById", workspace),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read a page of tracked application users using the documented pagination and filter arguments. Do not invent cursor fields. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.
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
    ///         .consumers
    ///         .page(
    ///             &"workspaceSlug".to_string(),
    ///             &PageConsumersRequest {
    ///                 args: PageConsumersRequestArgs {
    ///                     countries: None,
    ///                     end_user_org_ids: None,
    ///                     identity: None,
    ///                     last_seen_after: None,
    ///                     without_company: None,
    ///                     limit: 1.1,
    ///                     start: None,
    ///                     dir: PageConsumersRequestArgsDir::Forward,
    ///                     search: None,
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn page(
        &self,
        workspace: &str,
        request: &PageConsumersRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryConsumersGetPageQueryResponse, ApiError> {
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
                &format!("v3/workspaces/{}/queries/consumers.getPageQuery", workspace),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List tracked application users to resolve an internal user ID before fetching activity, devices or sessions. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.
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
    ///         .consumers
    ///         .list(
    ///             &"workspaceSlug".to_string(),
    ///             &ListConsumersRequest {
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
        request: &ListConsumersRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryConsumersListResponse, ApiError> {
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
                &format!("v3/workspaces/{}/queries/consumers.list", workspace),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns activity for users within the requested workspace and time range.
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
    ///         .consumers
    ///         .consumer_activity(
    ///             &"workspaceSlug".to_string(),
    ///             &ConsumerActivityRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn consumer_activity(
        &self,
        workspace: &str,
        request: &ConsumerActivityRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConsumerActivityResponse, ApiError> {
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
                &format!("v3/workspaces/{}/consumers/activity", workspace),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns the countries represented by the workspace's users.
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
    ///         .consumers
    ///         .consumer_countries(&"workspaceSlug".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn consumer_countries(
        &self,
        workspace: &str,
        options: Option<RequestOptions>,
    ) -> Result<ConsumerCountriesResponse, ApiError> {
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
                &format!("v3/workspaces/{}/consumers/countries", workspace),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns details for an application user in the workspace.
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
    ///         .consumers
    ///         .consumer_details(&"workspaceSlug".to_string(), &"endUserId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn consumer_details(
        &self,
        workspace: &str,
        end_user_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ConsumerDetailsResponse, ApiError> {
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
                &format!("v3/workspaces/{}/consumers/{}", workspace, end_user_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Finds an application user by email address or identifier and returns their profile, active problems, most recent error and recent sessions.
    ///
    /// # Arguments
    ///
    /// * `workspace` - Workspace URL slug, using lowercase letters, numbers, and hyphens.
    /// * `email` - Email address of the application user, compared case-insensitively. An exact match on the identifier your application sends with identify() also counts.
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
    ///         .consumers
    ///         .consumer_lookup(
    ///             &"workspaceSlug".to_string(),
    ///             &ConsumerLookupQueryRequest {
    ///                 email: "email".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn consumer_lookup(
        &self,
        workspace: &str,
        request: &ConsumerLookupQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConsumerLookupResponse, ApiError> {
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
                &format!("v3/workspaces/{}/consumers/lookup", workspace),
                None,
                QueryBuilder::new()
                    .string("email", request.email.clone())
                    .build(),
                options,
            )
            .await
    }
}
