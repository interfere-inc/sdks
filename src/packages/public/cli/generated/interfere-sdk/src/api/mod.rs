//! API client and types for the Interfere API
//!
//! This module contains all the API definitions including request/response types
//! and client implementations for interacting with the API.
//!
//! ## Modules
//!
//! - [`resources`] - Service clients and endpoints

pub mod resources;

pub use resources::{
    ApiClient, AuditClient, BillingClient, ConfigurationClient, ConsumersClient, DomainsClient,
    DrainClient, FeedbackClient, FindingsClient, IntegrationsClient, KeysClient, NetworkClient,
    NotificationsClient, ProblemsClient, ReleasesClient, ReplayClient, ScimClient, ServicesClient,
    SessionsClient, SsoClient, SupportClient, SurfaceKeysClient, SurfacesClient, TenantsClient,
    TiersClient, TracesClient, UsersClient, WorkspacesClient,
};

pub use interfere_types::*;
