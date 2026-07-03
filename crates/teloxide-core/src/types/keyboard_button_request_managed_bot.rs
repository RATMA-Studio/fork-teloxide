use serde::{Deserialize, Serialize};

/// This object defines the parameters for the creation of a managed bot.
///
/// Information about the created bot will be shared with the bot using the
/// update `managed_bot` and a Message with the field `managed_bot_created`.
///
/// [The official docs](https://core.telegram.org/bots/api#keyboardbuttonrequestmanagedbot).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct KeyboardButtonRequestManagedBot {
    /// Signed 32-bit identifier of the request. Must be unique within the
    /// message.
    pub request_id: i32,

    /// Suggested name for the bot.
    pub suggested_name: Option<String>,

    /// Suggested username for the bot.
    pub suggested_username: Option<String>
}
