use serde::{Deserialize, Serialize};

use crate::types::{PollId, PollOption};

/// Describes a service message about a new option added to a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#polloptionadded).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PollOptionAdded {
    /// Unique poll identifier.
    pub poll_id: PollId,

    /// Information about the added option.
    pub option: PollOption,
}
