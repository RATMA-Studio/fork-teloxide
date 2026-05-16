use mime::Mime;
use serde::{Deserialize, Serialize};

use crate::types::{FileMeta, PhotoSize, Seconds};

/// This object represents a live photo.
///
/// [The official docs](https://core.telegram.org/bots/api#livephoto).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct LivePhoto {
    /// Metadata of the live photo video file.
    #[serde(flatten)]
    pub file: FileMeta,

    /// Available sizes of the corresponding static photo.
    pub photo: Option<Vec<PhotoSize>>,

    /// Video width as defined by the sender.
    pub width: u32,

    /// Video height as defined by the sender.
    pub height: u32,

    /// Duration of the video in seconds as defined by the sender.
    pub duration: Seconds,

    /// MIME type of the file as defined by the sender.
    #[serde(with = "crate::types::non_telegram_types::mime::opt_deser")]
    #[cfg_attr(test, schemars(with = "Option<String>"))]
    pub mime_type: Option<Mime>,
}
