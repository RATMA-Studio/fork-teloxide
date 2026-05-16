use serde::{Deserialize, Serialize};

/// Describes an inline message sent by a guest bot.
///
/// Returned by the `answerGuestQuery` method.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct SentGuestMessage {
    /// Identifier of the sent inline message.
    pub inline_message_id: String,
}
