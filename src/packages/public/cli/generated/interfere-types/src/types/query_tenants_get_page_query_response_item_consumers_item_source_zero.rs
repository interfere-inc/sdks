pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryTenantsGetPageQueryResponseItemConsumersItemSourceZero {
    pub name: QueryTenantsGetPageQueryResponseItemConsumersItemSourceZeroName,
    pub r#type: QueryTenantsGetPageQueryResponseItemConsumersItemSourceZeroType,
}

impl QueryTenantsGetPageQueryResponseItemConsumersItemSourceZero {
    pub fn builder() -> QueryTenantsGetPageQueryResponseItemConsumersItemSourceZeroBuilder {
        <QueryTenantsGetPageQueryResponseItemConsumersItemSourceZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryTenantsGetPageQueryResponseItemConsumersItemSourceZeroBuilder {
    name: Option<QueryTenantsGetPageQueryResponseItemConsumersItemSourceZeroName>,
    r#type: Option<QueryTenantsGetPageQueryResponseItemConsumersItemSourceZeroType>,
}

impl QueryTenantsGetPageQueryResponseItemConsumersItemSourceZeroBuilder {
    pub fn name(mut self, value: QueryTenantsGetPageQueryResponseItemConsumersItemSourceZeroName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryTenantsGetPageQueryResponseItemConsumersItemSourceZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryTenantsGetPageQueryResponseItemConsumersItemSourceZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](QueryTenantsGetPageQueryResponseItemConsumersItemSourceZeroBuilder::name)
    /// - [`r#type`](QueryTenantsGetPageQueryResponseItemConsumersItemSourceZeroBuilder::r#type)
    pub fn build(self) -> Result<QueryTenantsGetPageQueryResponseItemConsumersItemSourceZero, BuildError> {
        Ok(QueryTenantsGetPageQueryResponseItemConsumersItemSourceZero {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
