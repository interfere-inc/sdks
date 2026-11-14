use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub mod settings;
pub use settings::SettingsClient;
pub struct NotificationsClient {
    pub http_client: HttpClient,
    pub settings: SettingsClient,
}

impl NotificationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            settings: SettingsClient::new(config.clone())?,
        })
    }

    /// Returns a token for the authenticated user's notification feed.
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
    ///     client.notifications.knock_token(None).await;
    /// }
    /// ```
    pub async fn knock_token(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<KnockTokenResponse, ApiError> {
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
            .execute_request(Method::GET, "v3/knock/token", None, None, options)
            .await
    }
}
