/*!
The KvStore store key/value pairs.
 */
// #![deny(missing_docs)]

mod common;
pub mod kvs;
pub use common::command::Command;
pub use common::error::{KVStoreError, Result};
pub use common::kv::KvStore;
