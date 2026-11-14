pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryConsumersListResponseItemSourceTwoName {
    #[serde(rename = "WorkOS")]
    WorkOs,
}
impl fmt::Display for QueryConsumersListResponseItemSourceTwoName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::WorkOs => "WorkOS",
        };
        write!(f, "{}", s)
    }
}
