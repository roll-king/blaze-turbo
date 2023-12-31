/*!
The KvStore store key/value pairs.
 */
// #![deny(missing_docs)]

mod common;

pub mod proto;
pub mod server;
pub use common::command::Command;
pub use common::error::{KVStoreError, Result};
pub use common::kv::KvStore;
pub use common::kv_engine::KvsEngine;
pub use common::sled::SledKvsEngine;
pub use proto::{Request, Response};
pub use server::{EngineType, KvServer};
