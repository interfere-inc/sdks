pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DetectIntegrationsResponseIntegrationsItemInstall {
    #[serde(rename = "operationId")]
    pub operation_id: DetectIntegrationsResponseIntegrationsItemInstallOperationId,
    #[serde(rename = "connectionId")]
    pub connection_id: DetectIntegrationsResponseIntegrationsItemInstallConnectionId,
}

impl DetectIntegrationsResponseIntegrationsItemInstall {
    pub fn builder() -> DetectIntegrationsResponseIntegrationsItemInstallBuilder {
        <DetectIntegrationsResponseIntegrationsItemInstallBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DetectIntegrationsResponseIntegrationsItemInstallBuilder {
    operation_id: Option<DetectIntegrationsResponseIntegrationsItemInstallOperationId>,
    connection_id: Option<DetectIntegrationsResponseIntegrationsItemInstallConnectionId>,
}

impl DetectIntegrationsResponseIntegrationsItemInstallBuilder {
    pub fn operation_id(mut self, value: DetectIntegrationsResponseIntegrationsItemInstallOperationId) -> Self {
        self.operation_id = Some(value);
        self
    }

    pub fn connection_id(mut self, value: DetectIntegrationsResponseIntegrationsItemInstallConnectionId) -> Self {
        self.connection_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DetectIntegrationsResponseIntegrationsItemInstall`].
    /// This method will fail if any of the following fields are not set:
    /// - [`operation_id`](DetectIntegrationsResponseIntegrationsItemInstallBuilder::operation_id)
    /// - [`connection_id`](DetectIntegrationsResponseIntegrationsItemInstallBuilder::connection_id)
    pub fn build(self) -> Result<DetectIntegrationsResponseIntegrationsItemInstall, BuildError> {
        Ok(DetectIntegrationsResponseIntegrationsItemInstall {
            operation_id: self.operation_id.ok_or_else(|| BuildError::missing_field("operation_id"))?,
            connection_id: self.connection_id.ok_or_else(|| BuildError::missing_field("connection_id"))?,
        })
    }
}
