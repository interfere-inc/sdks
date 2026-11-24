pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimelineEventContentNineConfirmationIdMetadata {
    /// Use a short natural title naming the affected capability and observed malfunction. Verbs are welcome: 'Replay recorder errors when another site is embedded' or 'Checkout loses saved delivery details'. Do not describe investigation activity or invent a failed user task. Never name code identifiers, error messages, exception names or status codes, the signal that revealed it, the library it came from, or the Surface or environment shown beside the Problem. Never quantify impact or scope: no counts, percentages or qualifiers such as 'isolated' or 'widespread'. Use everyday product language in titles, descriptions and opening summaries. Say 'had an error' or 'reported an error', not 'threw an exception'. Describe an iframe from another origin as 'another site embedded on the page', not 'embedded frames' or 'cross-origin windows'. Keep accurate API names, exception terminology and origin details in Observations and Technical details. Do not replace jargon with an unsupported user outcome. When you change it, set compactTitle in the same call; title and compactTitle must be updated together.
    #[serde(default)]
    pub title: String,
    /// Sidebar label naming the affected capability and observed malfunction in three or four plain words. Aim for about 24 characters, never more than 30 counting spaces: 'Checkout loses address' or 'Large uploads stall'. Drop words rather than squeeze a full title. Do not describe investigation activity or invent a failed user task. Never name code identifiers, error messages, exception names or status codes, the signal that revealed it, the library it came from, or the Surface or environment shown beside the Problem. Never quantify impact or scope: no counts, percentages or qualifiers such as 'isolated' or 'widespread'. Use everyday product language in titles, descriptions and opening summaries. Say 'had an error' or 'reported an error', not 'threw an exception'. Describe an iframe from another origin as 'another site embedded on the page', not 'embedded frames' or 'cross-origin windows'. Keep accurate API names, exception terminology and origin details in Observations and Technical details. Do not replace jargon with an unsupported user outcome. When you change it, set title in the same call; title and compactTitle must be updated together.
    #[serde(rename = "compactTitle")]
    #[serde(default)]
    pub compact_title: String,
    /// Explain what happened in one or two short sentences a product manager understands. Name the affected feature and a demonstrated trigger when known. Describe observed visible feedback and continued use separately when evidence supports them; otherwise leave them out. State only what was observed. Never add a sentence about what is unknown, unverified or not identified. Keep engineering specifics in expandable details. No severity claims, user counts, Surface or environment. Use everyday product language in titles, descriptions and opening summaries. Say 'had an error' or 'reported an error', not 'threw an exception'. Describe an iframe from another origin as 'another site embedded on the page', not 'embedded frames' or 'cross-origin windows'. Keep accurate API names, exception terminology and origin details in Observations and Technical details. Do not replace jargon with an unsupported user outcome. Report visible feedback separately from whether the page kept responding or a task completed. Check for an error message, toast, banner, dialog, browser alert or error page around the failure. Continued clicks, scrolling or recording do not prove the error was silent. An unhandled error flag describes code handling, not whether someone saw a message. Say 'silent' or 'no error was shown' only when the recorded evidence covers that claim. Sampled screenshots can miss a brief toast, native browser dialog, clipped viewport or recording gap. For limited samples say 'No error message is visible in the inspected replay frames' when true. Recorded navigations and interactions are outcome evidence even without replay frames: a full page load or redirect right after the failure replaced the page, and the next recorded click or navigation shows continued use. Report those with their timing and sample. If notification visibility is unknown, leave it out of titles and descriptions. Never turn an unobserved notification into harmlessness or successful task completion. Keep the sample limits when describing the outcome. No cause.
    #[serde(default)]
    pub description: String,
    /// Choose the affected capability: payment for checkout, sign-in for login, search for search, playback for media, layout for misplaced, overlapping or clipped page elements, network for connectivity. A browser exception or failed page render alone does not establish a layout defect. Prefer the specific affected capability over the page containing it. A programming error does not imply bug. Use bug only when the evidence identifies no more specific capability; use unknown when the affected capability is unknown. Never choose by browser, runtime or vendor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<TimelineEventContentNineConfirmationIdMetadataIcon>,
}

impl TimelineEventContentNineConfirmationIdMetadata {
    pub fn builder() -> TimelineEventContentNineConfirmationIdMetadataBuilder {
        <TimelineEventContentNineConfirmationIdMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentNineConfirmationIdMetadataBuilder {
    title: Option<String>,
    compact_title: Option<String>,
    description: Option<String>,
    icon: Option<TimelineEventContentNineConfirmationIdMetadataIcon>,
}

impl TimelineEventContentNineConfirmationIdMetadataBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn compact_title(mut self, value: impl Into<String>) -> Self {
        self.compact_title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn icon(mut self, value: TimelineEventContentNineConfirmationIdMetadataIcon) -> Self {
        self.icon = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentNineConfirmationIdMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](TimelineEventContentNineConfirmationIdMetadataBuilder::title)
    /// - [`compact_title`](TimelineEventContentNineConfirmationIdMetadataBuilder::compact_title)
    /// - [`description`](TimelineEventContentNineConfirmationIdMetadataBuilder::description)
    pub fn build(self) -> Result<TimelineEventContentNineConfirmationIdMetadata, BuildError> {
        Ok(TimelineEventContentNineConfirmationIdMetadata {
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            compact_title: self.compact_title.ok_or_else(|| BuildError::missing_field("compact_title"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            icon: self.icon,
        })
    }
}
