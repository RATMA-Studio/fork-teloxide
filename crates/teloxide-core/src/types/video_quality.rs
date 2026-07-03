use serde::{Deserialize, Serialize};

use crate::types::FileMeta;

/// This object represents a video file of a specific quality.
///
/// [The official docs](https://core.telegram.org/bots/api#videoquality).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct VideoQuality {
    /// Metadata of the video file.
    #[serde(flatten)]
    pub file: FileMeta,

    /// Video width.
    pub width: u32,

    /// Video height.
    pub height: u32,

    /// Codec that was used to encode the video, for example, `h264`, `h265`,
    /// or `av01`.
    pub codec: String
}
