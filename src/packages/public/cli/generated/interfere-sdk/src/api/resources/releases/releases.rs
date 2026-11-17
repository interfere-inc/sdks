use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ReleasesClient {
    pub http_client: HttpClient,
}

impl ReleasesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Atomically uploads source-map chunks from the signed inventory. Accepts Content-Encoding: gzip. The encoded and decoded JSON bodies must each fit within 8 MiB. Send the sign response's uploadGrant as the X-Interfere-Upload-Grant header in place of the API key.
    ///
    /// # Arguments
    ///
    /// * `environment` - Deploy environment of the build. Each environment of a release has its own source maps; omit it to use the source maps of the release.
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
    ///         .releases
    ///         .upload_source_map_batch(
    ///             &"releaseSlug".to_string(),
    ///             &SourceMapUploadBatch {
    ///                 environment: Some("production".to_string()),
    ///                 chunks: vec![SourceMapUploadBatchChunksItem {
    ///                     path: "path".to_string(),
    ///                     offset: 1,
    ///                     bytes: "bytes".to_string(),
    ///                     ..Default::default()
    ///                 }],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn upload_source_map_batch(
        &self,
        release_slug: &str,
        request: &SourceMapUploadBatch,
        options: Option<RequestOptions>,
    ) -> Result<SourceMapUploadBatchResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["apiKey"] as &[&str]])
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
                &format!("v3/releases/{}/source-maps/batch", release_slug),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("environment", request.environment.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Reads the exact environment upload session and verifies its published manifest without changing upload state.
    ///
    /// # Arguments
    ///
    /// * `environment` - Deploy environment of the build. Each environment of a release has its own source maps; omit it to use the source maps of the release.
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
    ///         .releases
    ///         .get_source_map_publication_status(
    ///             &"releaseSlug".to_string(),
    ///             &GetSourceMapPublicationStatusQueryRequest {
    ///                 environment: Some("production".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_source_map_publication_status(
        &self,
        release_slug: &str,
        request: &GetSourceMapPublicationStatusQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SourceMapPublicationStatus, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["apiKey"] as &[&str]])
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
                &format!("v3/releases/{}/source-maps", release_slug),
                None,
                QueryBuilder::new()
                    .string("environment", request.environment.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Uploads a source-map chunk declared to the sign call. Accepts Content-Encoding: gzip; encoded and decoded chunks must each fit within 8 MiB. Offsets refer to decoded bytes. Send the sign response's uploadGrant as the X-Interfere-Upload-Grant header in place of the API key.
    ///
    /// # Arguments
    ///
    /// * `environment` - Deploy environment of the build. Each environment of a release has its own source maps; omit it to use the source maps of the release.
    /// * `path` - Build-relative path of the source map, as declared to the sign call
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn upload_source_map(
        &self,
        release_slug: &str,
        request: &UploadSourceMapRequest,
        options: Option<RequestOptions>,
    ) -> Result<UploadSourceMapResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["apiKey"] as &[&str]])
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
            .execute_bytes_request(
                Method::PUT,
                &format!("v3/releases/{}/source-maps", release_slug),
                Some(request.body.to_vec()),
                QueryBuilder::new()
                    .string("environment", request.environment.clone())
                    .serialize("offset", request.offset.clone())
                    .string("path", request.path.clone())
                    .build(),
                "application/octet-stream",
                options,
            )
            .await
    }

    /// Resolves the surface and organization a build-time key belongs to.
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
    ///     client.releases.get_releases_config(None).await;
    /// }
    /// ```
    pub async fn get_releases_config(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<ReleasesConfigResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["apiKey"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(Method::GET, "v3/releases/config", None, None, options)
            .await
    }

    /// Registers a release for the surface the build-time key belongs to.
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
    ///         .releases
    ///         .create_release(
    ///             &CreateReleaseRequest {
    ///                 build_id: "0123456789abcdef0123456789abcdef01234567".to_string(),
    ///                 source: ReleaseSource::Github {
    ///                     data: GithubReleaseSource {
    ///                         branch: "branch".to_string(),
    ///                         commit_message: "commitMessage".to_string(),
    ///                         commit_sha: None,
    ///                         ..Default::default()
    ///                     },
    ///                 },
    ///                 destination: None,
    ///                 producer_version: None,
    ///                 service: None,
    ///                 slug: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_release(
        &self,
        request: &CreateReleaseRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateReleaseResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["apiKey"] as &[&str]])
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
                "v3/releases",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Initializes a source-map upload session and returns its expiry and file inventory.
    ///
    /// # Arguments
    ///
    /// * `environment` - Deploy environment of the build. Each environment of a release has its own source maps; omit it to use the source maps of the release.
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
    ///         .releases
    ///         .sign_source_map_uploads(
    ///             &"releaseSlug".to_string(),
    ///             &SignSourceMapsRequest {
    ///                 environment: Some("production".to_string()),
    ///                 files: vec![SignSourceMapFile {
    ///                     has_file: true,
    ///                     has_names: true,
    ///                     has_sources_content: true,
    ///                     mappings_present: true,
    ///                     path: "path".to_string(),
    ///                     size_bytes: 1,
    ///                     ..Default::default()
    ///                 }],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn sign_source_map_uploads(
        &self,
        release_slug: &str,
        request: &SignSourceMapsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SourceMapUploadSessionResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["apiKey"] as &[&str]])
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
                &format!("v3/releases/{}/source-maps/sign", release_slug),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("environment", request.environment.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Durably accepts all signed files and their verified manifest. R2 publication retries in the background.
    ///
    /// # Arguments
    ///
    /// * `environment` - Deploy environment of the build. Each environment of a release has its own source maps; omit it to use the source maps of the release.
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
    ///         .releases
    ///         .finalize_source_maps(
    ///             &"releaseSlug".to_string(),
    ///             &CompleteSourceMapsRequest {
    ///                 environment: Some("production".to_string()),
    ///                 bundler: ManifestBundler::Rollup,
    ///                 files: vec![CompleteSourceMapFile {
    ///                     chunk_url: "chunkUrl".to_string(),
    ///                     debug_id: "debugId".to_string(),
    ///                     hash: "hash".to_string(),
    ///                     path: "path".to_string(),
    ///                     ..Default::default()
    ///                 }],
    ///                 source_file_count: Some(0),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn finalize_source_maps(
        &self,
        release_slug: &str,
        request: &CompleteSourceMapsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CompleteSourceMapsResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["apiKey"] as &[&str]])
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
                &format!("v3/releases/{}/source-maps/finalize", release_slug),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("environment", request.environment.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Records source-map upload timings for a release.
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
    ///         .releases
    ///         .report_telemetry(
    ///             &"releaseSlug".to_string(),
    ///             &TelemetryRequest {
    ///                 cleanup: 1.1,
    ///                 create_release: 1.1,
    ///                 discover: 1.1,
    ///                 file_count: 1.1,
    ///                 total: 1.1,
    ///                 total_bytes: 1.1,
    ///                 upload: 1.1,
    ///                 preflight: None,
    ///                 result: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn report_telemetry(
        &self,
        release_slug: &str,
        request: &TelemetryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TelemetryResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["apiKey"] as &[&str]])
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
                &format!("v3/releases/{}/telemetry", release_slug),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List releases for an app (surface) to correlate a problem with deployed versions. Obtain the app slug from surfaces.list. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.
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
    ///         .releases
    ///         .list_for_surface(
    ///             &"workspaceSlug".to_string(),
    ///             &ListForSurfaceReleasesRequest {
    ///                 args: ListForSurfaceReleasesRequestArgs {
    ///                     surface_slug: "surfaceSlug".to_string(),
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
        request: &ListForSurfaceReleasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryReleasesForSurfaceResponse, ApiError> {
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
                &format!("v3/workspaces/{}/queries/releases.forSurface", workspace),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read the latest release for the selected app (surface) using the documented arguments. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.
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
    ///         .releases
    ///         .latest_by_surface(
    ///             &"workspaceSlug".to_string(),
    ///             &LatestBySurfaceReleasesRequest {
    ///                 args: LatestBySurfaceReleasesRequestArgs {
    ///                     surface_slug: "surfaceSlug".to_string(),
    ///                     primary_environment: None,
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn latest_by_surface(
        &self,
        workspace: &str,
        request: &LatestBySurfaceReleasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<Option<QueryReleasesLatestBySurfaceResponse>, ApiError> {
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
                    "v3/workspaces/{}/queries/releases.latestBySurface",
                    workspace
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read recent releases for the selected app (surface) to compare deployment timing with a regression. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.
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
    ///         .releases
    ///         .recent_by_surface(
    ///             &"workspaceSlug".to_string(),
    ///             &RecentBySurfaceReleasesRequest {
    ///                 args: RecentBySurfaceReleasesRequestArgs {
    ///                     surface_slug: "surfaceSlug".to_string(),
    ///                     limit: 1.1,
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn recent_by_surface(
        &self,
        workspace: &str,
        request: &RecentBySurfaceReleasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryReleasesRecentBySurfaceResponse, ApiError> {
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
                    "v3/workspaces/{}/queries/releases.recentBySurface",
                    workspace
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
