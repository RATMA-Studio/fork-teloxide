use serde::Serialize;

use crate::types::{
    InputFile, InputMediaAnimation, InputMediaLivePhoto, InputMediaLocation, InputMediaPhoto,
    InputMediaSticker, InputMediaVenue, InputMediaVideo,
};

/// Represents the content of a poll option to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputpolloptionmedia).
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InputPollOptionMedia {
    Animation(InputMediaAnimation),
    LivePhoto(InputMediaLivePhoto),
    Location(InputMediaLocation),
    Photo(InputMediaPhoto),
    Sticker(InputMediaSticker),
    Venue(InputMediaVenue),
    Video(InputMediaVideo),
}

#[allow(dead_code)]
impl InputPollOptionMedia {
    /// Returns an iterator of all files in this input poll-option media. Empty
    /// for non-file variants (`Location`, `Venue`).
    pub(crate) fn files(&self) -> impl Iterator<Item = &InputFile> {
        use InputPollOptionMedia::*;

        let pair: (Option<&InputFile>, Option<&InputFile>) = match self {
            Photo(InputMediaPhoto { media, .. }) | Sticker(InputMediaSticker { media, .. }) => {
                (Some(media), None)
            }
            Animation(InputMediaAnimation { media, thumbnail, .. })
            | Video(InputMediaVideo { media, thumbnail, .. }) => (Some(media), thumbnail.as_ref()),
            LivePhoto(InputMediaLivePhoto { media, photo, .. }) => (Some(media), Some(photo)),
            Location(_) | Venue(_) => (None, None),
        };

        pair.0.into_iter().chain(pair.1)
    }

    /// Returns an iterator of all files in this input poll-option media. Empty
    /// for non-file variants (`Location`, `Venue`).
    pub(crate) fn files_mut(&mut self) -> impl Iterator<Item = &mut InputFile> {
        use InputPollOptionMedia::*;

        let pair: (Option<&mut InputFile>, Option<&mut InputFile>) = match self {
            Photo(InputMediaPhoto { media, .. }) | Sticker(InputMediaSticker { media, .. }) => {
                (Some(media), None)
            }
            Animation(InputMediaAnimation { media, thumbnail, .. })
            | Video(InputMediaVideo { media, thumbnail, .. }) => (Some(media), thumbnail.as_mut()),
            LivePhoto(InputMediaLivePhoto { media, photo, .. }) => (Some(media), Some(photo)),
            Location(_) | Venue(_) => (None, None),
        };

        pair.0.into_iter().chain(pair.1)
    }
}
