use serde::{Deserialize, Serialize};

use crate::types::User;

/// This object describes the access settings of a bot.
///
/// [The official docs](https://core.telegram.org/bots/api#botaccesssettings).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct BotAccessSettings {
    /// `true`, if only selected users can access the bot. The bot's owner can
    /// always access it.
    pub is_access_restricted: bool,

    /// The list of other users who have access to the bot if the access is
    /// restricted.
    pub added_users: Option<Vec<User>>
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::UserId;

    #[test]
    fn deserialize_restricted_with_users() {
        let json = r#"{
            "is_access_restricted": true,
            "added_users": [
                {"id": 12345, "is_bot": false, "first_name": "Alice"}
            ]
        }"#;

        let s: BotAccessSettings = serde_json::from_str(json).unwrap();
        assert_eq!(s.is_access_restricted, true);
        let users = s.added_users.unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].id, UserId(12345));
        assert_eq!(users[0].first_name, "Alice");
    }

    #[test]
    fn deserialize_unrestricted() {
        let json = r#"{"is_access_restricted": false}"#;
        let s: BotAccessSettings = serde_json::from_str(json).unwrap();
        assert_eq!(s.is_access_restricted, false);
        assert!(s.added_users.is_none());
    }
}
