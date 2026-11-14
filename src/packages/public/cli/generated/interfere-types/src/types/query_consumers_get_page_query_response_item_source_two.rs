pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryConsumersGetPageQueryResponseItemSourceTwo {
    pub name: QueryConsumersGetPageQueryResponseItemSourceTwoName,
    pub r#type: QueryConsumersGetPageQueryResponseItemSourceTwoType,
}

impl QueryConsumersGetPageQueryResponseItemSourceTwo {
    pub fn builder() -> QueryConsumersGetPageQueryResponseItemSourceTwoBuilder {
        <QueryConsumersGetPageQueryResponseItemSourceTwoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryConsumersGetPageQueryResponseItemSourceTwoBuilder {
    name: Option<QueryConsumersGetPageQueryResponseItemSourceTwoName>,
    r#type: Option<QueryConsumersGetPageQueryResponseItemSourceTwoType>,
}

impl QueryConsumersGetPageQueryResponseItemSourceTwoBuilder {
    pub fn name(mut self, value: QueryConsumersGetPageQueryResponseItemSourceTwoName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryConsumersGetPageQueryResponseItemSourceTwoType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryConsumersGetPageQueryResponseItemSourceTwo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](QueryConsumersGetPageQueryResponseItemSourceTwoBuilder::name)
    /// - [`r#type`](QueryConsumersGetPageQueryResponseItemSourceTwoBuilder::r#type)
    pub fn build(self) -> Result<QueryConsumersGetPageQueryResponseItemSourceTwo, BuildError> {
        Ok(QueryConsumersGetPageQueryResponseItemSourceTwo {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
