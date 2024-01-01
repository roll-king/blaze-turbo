/// This module contains the thread pool implementations used for executing tasks in parallel.
/// It provides three different thread pool implementations: `NaiveThreadPool`, `RayonThreadPool`, and `SharedQueueThreadPool`.
/// Each thread pool implements the `ThreadPool` trait, which defines the common interface for thread pools.
use crate::Result;

mod naive_thread_pool;
mod rayon_thread_pool;
mod shared_queue_thread_pool;

pub use naive_thread_pool::NaiveThreadPool;
pub use rayon_thread_pool::RayonThreadPool;
pub use shared_queue_thread_pool::SharedQueueThreadPool;

/// A pool which uses multiple threads to execute tasks.
pub trait ThreadPool {
    /// Creates a new thread pool with the specified number of threads.
    /// The `threads` parameter determines the number of worker threads that will be spawned in the pool.
    /// If any thread fails to spawn, an error is returned and all previously-spawned threads are terminated.
    /// The returned `Result` contains the initialized thread pool on success.
    fn new(threads: usize) -> Result<Self>
    where
        Self: Sized;

    /// Spawns a function into the thread pool.
    /// The function is executed asynchronously by one of the worker threads in the pool.
    /// Spawning always succeeds, even if the function panics.
    /// If the function panics, the thread pool continues to operate with the same number of threads.
    /// The thread count is not reduced, and the thread pool remains valid and functional.
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;
}
