//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Drain**
//! - **configuration**
//! - **Traces**
//! - **Register builds and upload source maps**
//! - **Sessions**
//! - **Notifications**
//! - **Workspaces**
//! - **Manage workspace network access**
//! - **Inspect workspace audit events**
//! - **Domains**
//! - **Inspect companies using your application**
//! - **Consumers**
//! - **Manage workspace environments**
//! - **Inspect findings associated with a problem**
//! - **Integrations**
//! - **Manage workspace members, invitations, and your profile**
//! - **Problems**
//! - **Services**
//! - **Billing**
//! - **Manage the tiers that label companies**
//! - **Replay**
//! - **Feedback**
//! - **Surface keys**
//! - **Keys**
//! - **Scim**
//! - **Sso**
//! - **Support**

use crate::{ApiError, ClientConfig};

pub mod audit;
pub mod billing;
pub mod configuration;
pub mod consumers;
pub mod domains;
pub mod drain;
pub mod environments;
pub mod feedback;
pub mod findings;
pub mod integrations;
pub mod keys;
pub mod network;
pub mod notifications;
pub mod problems;
pub mod releases;
pub mod replay;
pub mod scim;
pub mod services;
pub mod sessions;
pub mod sso;
pub mod support;
pub mod surface_keys;
pub mod tenants;
pub mod tiers;
pub mod traces;
pub mod users;
pub mod workspaces;
pub struct ApiClient {
    pub config: ClientConfig,
    pub drain: DrainClient,
    pub configuration: ConfigurationClient,
    pub traces: TracesClient,
    pub releases: ReleasesClient,
    pub sessions: SessionsClient,
    pub notifications: NotificationsClient,
    pub workspaces: WorkspacesClient,
    pub network: NetworkClient,
    pub audit: AuditClient,
    pub domains: DomainsClient,
    pub tenants: TenantsClient,
    pub consumers: ConsumersClient,
    pub environments: EnvironmentsClient,
    pub findings: FindingsClient,
    pub integrations: IntegrationsClient,
    pub users: UsersClient,
    pub problems: ProblemsClient,
    pub services: ServicesClient,
    pub billing: BillingClient,
    pub tiers: TiersClient,
    pub replay: ReplayClient,
    pub feedback: FeedbackClient,
    pub surface_keys: SurfaceKeysClient,
    pub keys: KeysClient,
    pub scim: ScimClient,
    pub sso: SsoClient,
    pub support: SupportClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            drain: DrainClient::new(config.clone())?,
            configuration: ConfigurationClient::new(config.clone())?,
            traces: TracesClient::new(config.clone())?,
            releases: ReleasesClient::new(config.clone())?,
            sessions: SessionsClient::new(config.clone())?,
            notifications: NotificationsClient::new(config.clone())?,
            workspaces: WorkspacesClient::new(config.clone())?,
            network: NetworkClient::new(config.clone())?,
            audit: AuditClient::new(config.clone())?,
            domains: DomainsClient::new(config.clone())?,
            tenants: TenantsClient::new(config.clone())?,
            consumers: ConsumersClient::new(config.clone())?,
            environments: EnvironmentsClient::new(config.clone())?,
            findings: FindingsClient::new(config.clone())?,
            integrations: IntegrationsClient::new(config.clone())?,
            users: UsersClient::new(config.clone())?,
            problems: ProblemsClient::new(config.clone())?,
            services: ServicesClient::new(config.clone())?,
            billing: BillingClient::new(config.clone())?,
            tiers: TiersClient::new(config.clone())?,
            replay: ReplayClient::new(config.clone())?,
            feedback: FeedbackClient::new(config.clone())?,
            surface_keys: SurfaceKeysClient::new(config.clone())?,
            keys: KeysClient::new(config.clone())?,
            scim: ScimClient::new(config.clone())?,
            sso: SsoClient::new(config.clone())?,
            support: SupportClient::new(config.clone())?,
        })
    }
}

pub use audit::AuditClient;
pub use billing::BillingClient;
pub use configuration::ConfigurationClient;
pub use consumers::ConsumersClient;
pub use domains::DomainsClient;
pub use drain::DrainClient;
pub use environments::EnvironmentsClient;
pub use feedback::FeedbackClient;
pub use findings::FindingsClient;
pub use integrations::IntegrationsClient;
pub use keys::KeysClient;
pub use network::NetworkClient;
pub use notifications::NotificationsClient;
pub use problems::ProblemsClient;
pub use releases::ReleasesClient;
pub use replay::ReplayClient;
pub use scim::ScimClient;
pub use services::ServicesClient;
pub use sessions::SessionsClient;
pub use sso::SsoClient;
pub use support::SupportClient;
pub use surface_keys::SurfaceKeysClient;
pub use tenants::TenantsClient;
pub use tiers::TiersClient;
pub use traces::TracesClient;
pub use users::UsersClient;
pub use workspaces::WorkspacesClient;
