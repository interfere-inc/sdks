use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SurfaceKeysClient {
    pub http_client: HttpClient,
}

impl SurfaceKeysClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Do not assume a found Interfere key works. Call this for every public or secret key found in environment variables, .env files, Worker secrets or CI secrets before reusing it. The key is checked live against the same authentication the SDKs and build plugins use and compared with this workspace; it is never stored, logged or returned. Act on outcome and nextStep: only `valid` means the key can be reused here.
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
    ///         .surface_keys
    ///         .validate_key(
    ///             &"workspaceSlug".to_string(),
    ///             &ValidateKeyRequest {
    ///                 key: "key".to_string(),
    ///                 expected: None,
    ///                 surface_public_key: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn validate_key(
        &self,
        workspace: &str,
        request: &ValidateKeyRequest,
        options: Option<RequestOptions>,
    ) -> Result<ValidateKeyResponse, ApiError> {
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
                &format!("v3/workspaces/{}/keys/validate", workspace),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
