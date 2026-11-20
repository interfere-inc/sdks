use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ServicesClient {
    pub http_client: HttpClient,
}

impl ServicesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the latest span time for each OpenTelemetry service in the workspace over the last 7 days. Services with no spans in that window are omitted. Also returns hourly span, log and metric point counts per service over the last 24 hours, aligned to `buckets`; services with no signals in that window are omitted from `signals`. `sdks` lists the Interfere SDK name and version that sent each namespace-less service's spans over the same 24 hours, with span counts. Each row’s `status` is `supported`, `outdated` or `unknown` for the newest SDK version that service reported, against the minimum supported SDK version.
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
    ///         .services
    ///         .service_activity(&"workspaceSlug".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn service_activity(
        &self,
        workspace: &str,
        options: Option<RequestOptions>,
    ) -> Result<ServiceActivityResponse, ApiError> {
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
                &format!("v3/workspaces/{}/services/activity", workspace),
                None,
                None,
                options,
            )
            .await
    }

    /// Deletes an OpenTelemetry service and its release history: releases, release deployments and source-map uploads. A service that apps, keys, replay sessions, findings, problems or a destination integration still refer to answers SERVICE_IN_USE, whose details list those blockers. Stored telemetry is kept, and the service is created again when telemetry with the same namespace, name and resource arrives.
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
    ///         .services
    ///         .delete_service(
    ///             &"workspaceSlug".to_string(),
    ///             &DeleteServiceRequest {
    ///                 name: "name".to_string(),
    ///                 namespace: None,
    ///                 resource: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_service(
        &self,
        workspace: &str,
        request: &DeleteServiceRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeleteServiceResponse, ApiError> {
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
                Method::DELETE,
                &format!("v3/workspaces/{}/services", workspace),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Sets the environment whose exceptions create findings and problems for an OpenTelemetry service, compared with the deployment.environment.name resource attribute. Exceptions from other or missing environments are still stored as telemetry. Null, the default, reports every environment.
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
    ///         .services
    ///         .set_service_reportable_environment(
    ///             &"workspaceSlug".to_string(),
    ///             &SetServiceReportableEnvironmentRequest {
    ///                 name: "name".to_string(),
    ///                 namespace: None,
    ///                 resource: None,
    ///                 reportable_environment: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn set_service_reportable_environment(
        &self,
        workspace: &str,
        request: &SetServiceReportableEnvironmentRequest,
        options: Option<RequestOptions>,
    ) -> Result<SetServiceReportableEnvironmentResponse, ApiError> {
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
                Method::PUT,
                &format!(
                    "v3/workspaces/{}/services/reportable-environment",
                    workspace
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Links an OpenTelemetry service to the repository Datasource and optional working directory that hold its code, or removes the link when source is null. Investigations read the commit from each Fact's service.version; no app or release is created.
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
    ///         .services
    ///         .set_service_source(
    ///             &"workspaceSlug".to_string(),
    ///             &SetServiceSourceRequest {
    ///                 name: "name".to_string(),
    ///                 namespace: None,
    ///                 resource: None,
    ///                 source: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn set_service_source(
        &self,
        workspace: &str,
        request: &SetServiceSourceRequest,
        options: Option<RequestOptions>,
    ) -> Result<SetServiceSourceResponse, ApiError> {
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
                Method::PUT,
                &format!("v3/workspaces/{}/services/source", workspace),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
