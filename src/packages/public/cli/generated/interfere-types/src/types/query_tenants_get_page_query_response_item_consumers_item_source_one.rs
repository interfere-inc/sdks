pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryTenantsGetPageQueryResponseItemConsumersItemSourceOne {
    pub name: QueryTenantsGetPageQueryResponseItemConsumersItemSourceOneName,
    pub r#type: QueryTenantsGetPageQueryResponseItemConsumersItemSourceOneType,
}

impl QueryTenantsGetPageQueryResponseItemConsumersItemSourceOne {
    pub fn builder() -> QueryTenantsGetPageQueryResponseItemConsumersItemSourceOneBuilder {
        <QueryTenantsGetPageQueryResponseItemConsumersItemSourceOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryTenantsGetPageQueryResponseItemConsumersItemSourceOneBuilder {
    name: Option<QueryTenantsGetPageQueryResponseItemConsumersItemSourceOneName>,
    r#type: Option<QueryTenantsGetPageQueryResponseItemConsumersItemSourceOneType>,
}

impl QueryTenantsGetPageQueryResponseItemConsumersItemSourceOneBuilder {
    pub fn name(mut self, value: QueryTenantsGetPageQueryResponseItemConsumersItemSourceOneName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryTenantsGetPageQueryResponseItemConsumersItemSourceOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryTenantsGetPageQueryResponseItemConsumersItemSourceOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](QueryTenantsGetPageQueryResponseItemConsumersItemSourceOneBuilder::name)
    /// - [`r#type`](QueryTenantsGetPageQueryResponseItemConsumersItemSourceOneBuilder::r#type)
    pub fn build(self) -> Result<QueryTenantsGetPageQueryResponseItemConsumersItemSourceOne, BuildError> {
        Ok(QueryTenantsGetPageQueryResponseItemConsumersItemSourceOne {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
