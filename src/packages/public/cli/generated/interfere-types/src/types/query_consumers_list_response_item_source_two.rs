pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryConsumersListResponseItemSourceTwo {
    pub name: QueryConsumersListResponseItemSourceTwoName,
    pub r#type: QueryConsumersListResponseItemSourceTwoType,
}

impl QueryConsumersListResponseItemSourceTwo {
    pub fn builder() -> QueryConsumersListResponseItemSourceTwoBuilder {
        <QueryConsumersListResponseItemSourceTwoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryConsumersListResponseItemSourceTwoBuilder {
    name: Option<QueryConsumersListResponseItemSourceTwoName>,
    r#type: Option<QueryConsumersListResponseItemSourceTwoType>,
}

impl QueryConsumersListResponseItemSourceTwoBuilder {
    pub fn name(mut self, value: QueryConsumersListResponseItemSourceTwoName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryConsumersListResponseItemSourceTwoType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryConsumersListResponseItemSourceTwo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](QueryConsumersListResponseItemSourceTwoBuilder::name)
    /// - [`r#type`](QueryConsumersListResponseItemSourceTwoBuilder::r#type)
    pub fn build(self) -> Result<QueryConsumersListResponseItemSourceTwo, BuildError> {
        Ok(QueryConsumersListResponseItemSourceTwo {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
