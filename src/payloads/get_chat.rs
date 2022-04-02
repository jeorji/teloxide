// This file is auto generated by [`cg`] from [`schema`].
//
// **DO NOT EDIT THIS FILE**,
//
// Edit `cg` or `schema` instead.
//
// [cg]: https://github.com/teloxide/cg
// [`schema`]: https://github.com/WaffleLapkin/tg-methods-schema
use serde::Serialize;

use crate::types::{Chat, Recipient};

impl_payload! {
    /// Use this method to get up to date information about the chat (current name of the user for one-on-one conversations, current username of a user, group or channel, etc.). Returns a [`Chat`] object on success.
    ///
    /// [`Chat`]: crate::types::Chat
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub GetChat (GetChatSetters) => Chat {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: Recipient [into],
        }
    }
}
