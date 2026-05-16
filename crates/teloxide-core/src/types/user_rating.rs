use serde::{Deserialize, Serialize};

/// This object describes the rating of a user based on their Telegram Star
/// spendings.
///
/// [The official docs](https://core.telegram.org/bots/api#userrating).
#[serde_with::skip_serializing_none]
#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UserRating {
    /// Current level of the user, indicating their reliability when purchasing
    /// digital goods and services. A higher level suggests a more trustworthy
    /// customer; a negative level is likely reason for concern.
    pub level: i32,

    /// Numerical value of the user's rating; the higher the rating, the
    /// better.
    pub rating: u32,

    /// The rating value required to get the current level.
    pub current_level_rating: u32,

    /// The rating value required to get to the next level; omitted if the
    /// maximum level was reached.
    pub next_level_rating: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let json = r#"{
            "level": 3,
            "rating": 1250,
            "current_level_rating": 1000,
            "next_level_rating": 2000
        }"#;

        let r: UserRating = serde_json::from_str(json).unwrap();
        assert_eq!(
            r,
            UserRating { level: 3, rating: 1250, current_level_rating: 1000, next_level_rating: Some(2000) }
        );
    }

    #[test]
    fn deserialize_negative_max_level() {
        let json =
            r#"{"level":-1,"rating":0,"current_level_rating":0}"#;
        let r: UserRating = serde_json::from_str(json).unwrap();
        assert_eq!(r.level, -1);
        assert_eq!(r.next_level_rating, None);
    }
}
