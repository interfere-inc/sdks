use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub mod invoices;
pub use invoices::InvoicesClient;
pub struct BillingClient {
    pub http_client: HttpClient,
    pub invoices: InvoicesClient,
}

impl BillingClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            invoices: InvoicesClient::new(config.clone())?,
        })
    }

    /// Returns a checkout session for the workspace. An existing active subscription prevents checkout.
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
    ///         .billing
    ///         .create_billing_checkout(
    ///             &"workspaceSlug".to_string(),
    ///             &CreateBillingCheckoutRequest {
    ///                 billing_interval: CreateBillingCheckoutRequestBillingInterval::Monthly,
    ///                 intent_id: "intentId".to_string(),
    ///                 success_url: "successUrl".to_string(),
    ///                 cancel_url: None,
    ///                 plan: None,
    ///                 promotion_code: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_billing_checkout(
        &self,
        workspace: &str,
        request: &CreateBillingCheckoutRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateBillingCheckoutResponse, ApiError> {
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
                &format!("v3/workspaces/{}/billing/checkout", workspace),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a billing portal session for the workspace's billing customer.
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
    ///         .billing
    ///         .create_billing_portal(
    ///             &"workspaceSlug".to_string(),
    ///             &CreateBillingPortalRequest {
    ///                 intent_id: "intentId".to_string(),
    ///                 return_url: "returnUrl".to_string(),
    ///                 flow: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_billing_portal(
        &self,
        workspace: &str,
        request: &CreateBillingPortalRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateBillingPortalResponse, ApiError> {
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
                &format!("v3/workspaces/{}/billing/portal", workspace),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
