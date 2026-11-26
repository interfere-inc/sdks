pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SetAnonymousReplayPolicyServicesRequestArgs {
    #[serde(rename = "serviceId")]
    #[serde(default)]
    pub service_id: String,
    /// When sessions without an identified user record replay: always, only once an error occurs, or never.
    pub policy: SetAnonymousReplayPolicyServicesRequestArgsPolicy,
}

impl SetAnonymousReplayPolicyServicesRequestArgs {
    pub fn builder() -> SetAnonymousReplayPolicyServicesRequestArgsBuilder {
        <SetAnonymousReplayPolicyServicesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetAnonymousReplayPolicyServicesRequestArgsBuilder {
    service_id: Option<String>,
    policy: Option<SetAnonymousReplayPolicyServicesRequestArgsPolicy>,
}

impl SetAnonymousReplayPolicyServicesRequestArgsBuilder {
    pub fn service_id(mut self, value: impl Into<String>) -> Self {
        self.service_id = Some(value.into());
        self
    }

    pub fn policy(mut self, value: SetAnonymousReplayPolicyServicesRequestArgsPolicy) -> Self {
        self.policy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetAnonymousReplayPolicyServicesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`service_id`](SetAnonymousReplayPolicyServicesRequestArgsBuilder::service_id)
    /// - [`policy`](SetAnonymousReplayPolicyServicesRequestArgsBuilder::policy)
    pub fn build(self) -> Result<SetAnonymousReplayPolicyServicesRequestArgs, BuildError> {
        Ok(SetAnonymousReplayPolicyServicesRequestArgs {
            service_id: self.service_id.ok_or_else(|| BuildError::missing_field("service_id"))?,
            policy: self.policy.ok_or_else(|| BuildError::missing_field("policy"))?,
        })
    }
}
