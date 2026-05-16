use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Describes a keyboard button that can be added by the bot to a reply
/// keyboard of an opener of a Mini App.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PreparedKeyboardButton {
    /// Unique identifier of the prepared button.
    pub id: String,

    /// Expiration date of the prepared button, in Unix time. Expired prepared
    /// buttons can no longer be used.
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    #[cfg_attr(test, schemars(with = "i64"))]
    pub expiration_date: DateTime<Utc>,
}
