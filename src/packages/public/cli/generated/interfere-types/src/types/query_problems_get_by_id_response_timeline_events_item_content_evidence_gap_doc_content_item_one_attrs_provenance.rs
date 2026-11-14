pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOneAttrsProvenance {
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOneAttrsProvenance {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOneAttrsProvenanceBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOneAttrsProvenanceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOneAttrsProvenanceBuilder {
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOneAttrsProvenanceBuilder {

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOneAttrsProvenance`].
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOneAttrsProvenance, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOneAttrsProvenance {
        })
    }
}
