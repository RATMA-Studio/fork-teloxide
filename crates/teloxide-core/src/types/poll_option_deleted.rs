use serde::{Deserialize, Serialize};

use crate::types::{MaybeInaccessibleMessage, MessageEntity};

/// Describes a service message about an option deleted from a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#polloptiondeleted).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PollOptionDeleted {
    /// Message containing the poll from which the option was deleted, if
    /// known. Note that the inner `Message` object in this field will not
    /// contain the `reply_to_message` field even if it itself is a reply.
    pub poll_message: Option<MaybeInaccessibleMessage>,

    /// Persistent identifier of the deleted option.
    pub option_persistent_id: String,

    /// Option text.
    pub option_text: String,

    /// Special entities that appear in the [`option_text`].
    ///
    /// [`option_text`]: PollOptionDeleted::option_text
    pub option_text_entities: Option<Vec<MessageEntity>>,
}
