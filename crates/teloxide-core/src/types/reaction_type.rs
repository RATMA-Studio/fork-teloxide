use serde::{Deserialize, Serialize};

use crate::types::CustomEmojiId;

/// The reaction type is based on an emoji or custom emoji.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ReactionType {
    /// Emoji reaction.
    Emoji {
        /// Reaction emoji. Currently, it can be one of "👍", "👎", "❤", "🔥",
        /// "🥰", "👏", "😁", "🤔", "🤯", "😱", "🤬", "😢", "🎉", "🤩",
        /// "🤮", "💩", "🙏", "👌", "🕊", "🤡", "🥱", "🥴", "😍", "🐳",
        /// "❤‍🔥", "🌚", "🌭", "💯", "🤣", "⚡", "🍌", "🏆", "💔", "🤨",
        /// "😐", "🍓", "🍾", "💋", "🖕", "😈", "😴", "😭", "🤓", "👻",
        /// "👨‍💻", "👀", "🎃", "🙈", "😇", "😨", "🤝", "✍", "🤗", "🫡",
        /// "🎅", "🎄", "☃", "💅", "🤪", "🗿", "🆒", "💘", "🙉", "🦄", "😘",
        /// "💊", "🙊", "😎", "👾", "🤷‍♂", "🤷", "🤷‍♀", "😡"
        emoji: String
    },
    /// Custom emoji reaction.
    CustomEmoji {
        /// Custom emoji identifier.
        custom_emoji_id: CustomEmojiId
    },
    /// Paid reaction.
    Paid
}

impl ReactionType {
    #[must_use]
    pub fn emoji(&self) -> Option<&String> {
        match &self {
            Self::Emoji {
                emoji
            } => Some(emoji),
            _ => None
        }
    }

    #[must_use]
    pub fn custom_emoji_id(&self) -> Option<&CustomEmojiId> {
        match &self {
            Self::CustomEmoji {
                custom_emoji_id
            } => Some(custom_emoji_id),
            _ => None
        }
    }
}
