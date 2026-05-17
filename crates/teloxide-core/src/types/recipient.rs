use derive_more::{Display, From};
use serde::{Deserialize, Serialize};

use crate::types::{ChatId, UserId};

/// A unique identifier for the target chat or username of the target channel
/// or bot (in the format `@username`).
///
/// Since Bot API 10.0 messages can also be sent to other bots via their
/// `@bot_username` if both bots enabled bot-to-bot communication.
#[derive(Clone, PartialEq, Eq, Hash)]
#[derive(Debug, Display, From)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(untagged)]
#[non_exhaustive]
pub enum Recipient {
    /// A chat identifier.
    #[display("{_0}")]
    Id(ChatId),

    /// A public username (in the format `@username`). Can be a channel,
    /// supergroup or, since Bot API 10.0, a bot when bot-to-bot
    /// communication is enabled on both sides.
    #[display("{_0}")]
    ChannelUsername(String),
}

impl From<UserId> for Recipient {
    fn from(id: UserId) -> Self {
        Self::Id(id.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_id_id_serialization() {
        let expected_json = String::from("123456");
        let actual_json = serde_json::to_string(&Recipient::Id(ChatId(123_456))).unwrap();

        assert_eq!(expected_json, actual_json)
    }

    #[test]
    fn chat_id_channel_username_serialization() {
        let expected_json = String::from(r#""@username""#);
        let actual_json =
            serde_json::to_string(&Recipient::ChannelUsername(String::from("@username"))).unwrap();

        assert_eq!(expected_json, actual_json)
    }
}
