use serde::Serialize;

use crate::types::{
    InputFile, InputMediaAnimation, InputMediaAudio, InputMediaDocument, InputMediaLivePhoto,
    InputMediaLocation, InputMediaPhoto, InputMediaVenue, InputMediaVideo,
};

/// Represents the content of a poll description or a quiz explanation to be
/// sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputpollmedia).
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InputPollMedia {
    Animation(InputMediaAnimation),
    Audio(InputMediaAudio),
    Document(InputMediaDocument),
    LivePhoto(InputMediaLivePhoto),
    Location(InputMediaLocation),
    Photo(InputMediaPhoto),
    Venue(InputMediaVenue),
    Video(InputMediaVideo),
}

#[allow(dead_code)]
impl InputPollMedia {
    /// Returns an iterator of all files in this input poll media. Empty for
    /// non-file variants (`Location`, `Venue`).
    pub(crate) fn files(&self) -> impl Iterator<Item = &InputFile> {
        use InputPollMedia::*;

        let pair: (Option<&InputFile>, Option<&InputFile>) = match self {
            Photo(InputMediaPhoto { media, .. }) => (Some(media), None),
            Document(InputMediaDocument { media, thumbnail, .. })
            | Audio(InputMediaAudio { media, thumbnail, .. })
            | Animation(InputMediaAnimation { media, thumbnail, .. })
            | Video(InputMediaVideo { media, thumbnail, .. }) => (Some(media), thumbnail.as_ref()),
            LivePhoto(InputMediaLivePhoto { media, photo, .. }) => (Some(media), Some(photo)),
            Location(_) | Venue(_) => (None, None),
        };

        pair.0.into_iter().chain(pair.1)
    }

    /// Returns an iterator of all files in this input poll media. Empty for
    /// non-file variants (`Location`, `Venue`).
    pub(crate) fn files_mut(&mut self) -> impl Iterator<Item = &mut InputFile> {
        use InputPollMedia::*;

        let pair: (Option<&mut InputFile>, Option<&mut InputFile>) = match self {
            Photo(InputMediaPhoto { media, .. }) => (Some(media), None),
            Document(InputMediaDocument { media, thumbnail, .. })
            | Audio(InputMediaAudio { media, thumbnail, .. })
            | Animation(InputMediaAnimation { media, thumbnail, .. })
            | Video(InputMediaVideo { media, thumbnail, .. }) => (Some(media), thumbnail.as_mut()),
            LivePhoto(InputMediaLivePhoto { media, photo, .. }) => (Some(media), Some(photo)),
            Location(_) | Venue(_) => (None, None),
        };

        pair.0.into_iter().chain(pair.1)
    }
}
