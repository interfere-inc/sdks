pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConsumerCountriesResponse {
    #[serde(default)]
    pub countries: Vec<String>,
}

impl ConsumerCountriesResponse {
    pub fn builder() -> ConsumerCountriesResponseBuilder {
        <ConsumerCountriesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConsumerCountriesResponseBuilder {
    countries: Option<Vec<String>>,
}

impl ConsumerCountriesResponseBuilder {
    pub fn countries(mut self, value: Vec<String>) -> Self {
        self.countries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConsumerCountriesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`countries`](ConsumerCountriesResponseBuilder::countries)
    pub fn build(self) -> Result<ConsumerCountriesResponse, BuildError> {
        Ok(ConsumerCountriesResponse {
            countries: self.countries.ok_or_else(|| BuildError::missing_field("countries"))?,
        })
    }
}
