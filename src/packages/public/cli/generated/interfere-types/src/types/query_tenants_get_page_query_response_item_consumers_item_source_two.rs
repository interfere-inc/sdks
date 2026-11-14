pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryTenantsGetPageQueryResponseItemConsumersItemSourceTwo {
    pub name: QueryTenantsGetPageQueryResponseItemConsumersItemSourceTwoName,
    pub r#type: QueryTenantsGetPageQueryResponseItemConsumersItemSourceTwoType,
}

impl QueryTenantsGetPageQueryResponseItemConsumersItemSourceTwo {
    pub fn builder() -> QueryTenantsGetPageQueryResponseItemConsumersItemSourceTwoBuilder {
        <QueryTenantsGetPageQueryResponseItemConsumersItemSourceTwoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryTenantsGetPageQueryResponseItemConsumersItemSourceTwoBuilder {
    name: Option<QueryTenantsGetPageQueryResponseItemConsumersItemSourceTwoName>,
    r#type: Option<QueryTenantsGetPageQueryResponseItemConsumersItemSourceTwoType>,
}

impl QueryTenantsGetPageQueryResponseItemConsumersItemSourceTwoBuilder {
    pub fn name(mut self, value: QueryTenantsGetPageQueryResponseItemConsumersItemSourceTwoName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryTenantsGetPageQueryResponseItemConsumersItemSourceTwoType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryTenantsGetPageQueryResponseItemConsumersItemSourceTwo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](QueryTenantsGetPageQueryResponseItemConsumersItemSourceTwoBuilder::name)
    /// - [`r#type`](QueryTenantsGetPageQueryResponseItemConsumersItemSourceTwoBuilder::r#type)
    pub fn build(self) -> Result<QueryTenantsGetPageQueryResponseItemConsumersItemSourceTwo, BuildError> {
        Ok(QueryTenantsGetPageQueryResponseItemConsumersItemSourceTwo {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
