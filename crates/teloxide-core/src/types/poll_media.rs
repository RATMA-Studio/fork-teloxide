use serde::{Deserialize, Serialize};

use crate::types::{
    Animation, Audio, Document, LivePhoto, Location, PhotoSize, Sticker, Venue, Video
};

/// Describes a media added to a poll, poll option, or quiz explanation.
///
/// At most one of the optional fields can be present in any given object.
///
/// [The official docs](https://core.telegram.org/bots/api#pollmedia).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PollMedia {
    /// Media is an animation, information about the animation.
    pub animation: Option<Animation>,

    /// Media is an audio file, information about the file; currently, can't be
    /// received in a poll option.
    pub audio: Option<Audio>,

    /// Media is a general file, information about the file; currently, can't be
    /// received in a poll option.
    pub document: Option<Document>,

    /// Media is a live photo, information about the live photo.
    pub live_photo: Option<LivePhoto>,

    /// Media is a shared location, information about the location.
    pub location: Option<Location>,

    /// Media is a photo, available sizes of the photo.
    pub photo: Option<Vec<PhotoSize>>,

    /// Media is a sticker, information about the sticker; currently, for poll
    /// options only.
    pub sticker: Option<Sticker>,

    /// Media is a venue, information about the venue.
    pub venue: Option<Venue>,

    /// Media is a video, information about the video.
    pub video: Option<Video>
}
