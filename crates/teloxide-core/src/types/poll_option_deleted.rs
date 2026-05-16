use serde::{Deserialize, Serialize};

use crate::types::PollId;

/// Describes a service message about an option deleted from a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#polloptiondeleted).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PollOptionDeleted {
    /// Unique poll identifier.
    pub poll_id: PollId,

    /// Persistent identifier of the deleted option.
    pub option_persistent_id: String,
}
