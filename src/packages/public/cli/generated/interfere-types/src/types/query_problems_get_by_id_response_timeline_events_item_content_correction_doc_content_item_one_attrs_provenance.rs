pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOneAttrsProvenance {
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOneAttrsProvenance {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOneAttrsProvenanceBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOneAttrsProvenanceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOneAttrsProvenanceBuilder {
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOneAttrsProvenanceBuilder {

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOneAttrsProvenance`].
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOneAttrsProvenance, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDocContentItemOneAttrsProvenance {
        })
    }
}
