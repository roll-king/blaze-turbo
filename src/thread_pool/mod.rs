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
    /// Creates a new thread pool, immediately spawning the specified number of threads.
    /// Returns an error if any thread fails to spawn. All previously-spawned threads are terminated.
    fn new(threads: usize) -> Result<Self>
    where
        Self: Sized;

    /// Spawns a function into the thread pool.
    /// Spawning always succeeds, but if the function panics, the thread pool continues to operate with the same number of threads.
    /// The thread count is not reduced, and the thread pool is not destroyed, corrupted, or invalidated.
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;
}
