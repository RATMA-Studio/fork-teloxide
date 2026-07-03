use serde::{Deserialize, Serialize};

use crate::types::{MaybeInaccessibleMessage, MessageEntity};

/// Describes a service message about a new option added to a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#polloptionadded).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PollOptionAdded {
    /// Message containing the poll to which the option was added, if known.
    /// Note that the inner `Message` object in this field will not contain
    /// the `reply_to_message` field even if it itself is a reply.
    pub poll_message: Option<MaybeInaccessibleMessage>,

    /// Persistent identifier of the added option.
    pub option_persistent_id: String,

    /// Option text.
    pub option_text: String,

    /// Special entities that appear in the [`option_text`].
    ///
    /// [`option_text`]: PollOptionAdded::option_text
    pub option_text_entities: Option<Vec<MessageEntity>>
}
