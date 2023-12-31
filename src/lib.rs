#![deny(missing_docs)]
/*!
The KvStore store key/value pairs.
 */
mod client;
mod common;
mod proto;
mod server;
mod thread_pool;

pub use client::Client;
pub use common::error::{KVStoreError, Result};
pub use common::Command;
pub use common::{KvStore, KvsEngine, SledKvsEngine};
pub use proto::{Request, Response};
pub use server::{EngineType, KvServer};
pub use thread_pool::{NaiveThreadPool, RayonThreadPool, SharedQueueThreadPool, ThreadPool};
