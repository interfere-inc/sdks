pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArguments {
    pub kind: TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArgumentsKind,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub arguments: HashMap<String, Option<TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArgumentsArgumentsValue>>,
}

impl TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArguments {
    pub fn builder() -> TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArgumentsBuilder {
        <TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArgumentsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArgumentsBuilder {
    kind: Option<TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArgumentsKind>,
    name: Option<String>,
    arguments: Option<HashMap<String, Option<TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArgumentsArgumentsValue>>>,
}

impl TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArgumentsBuilder {
    pub fn kind(mut self, value: TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArgumentsKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn arguments(mut self, value: HashMap<String, Option<TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArgumentsArgumentsValue>>) -> Self {
        self.arguments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArguments`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArgumentsBuilder::kind)
    /// - [`name`](TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArgumentsBuilder::name)
    /// - [`arguments`](TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArgumentsBuilder::arguments)
    pub fn build(self) -> Result<TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArguments, BuildError> {
        Ok(TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArguments {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            arguments: self.arguments.ok_or_else(|| BuildError::missing_field("arguments"))?,
        })
    }
}
