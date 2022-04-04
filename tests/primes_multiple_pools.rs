#![feature(scoped_threads)]
mod common;

use scoped_threadpool::thread_pool::ThreadPool;

const MAX_CANDIDATE: u64 = 100000;
const NUM_THREADS_PER_POOL: usize = 8;
const NUM_THREADPOOLS: usize = 10;

#[test]
pub fn primes_multiple_pools() {
    std::thread::scope(|scope| {
        let mut pools: Vec<ThreadPool> = Vec::with_capacity(NUM_THREADPOOLS);
        for _ in 0..NUM_THREADPOOLS {
            pools.push(ThreadPool::new(NUM_THREADS_PER_POOL, scope))
        }
        for i in 2..MAX_CANDIDATE {
            pools[i as usize % NUM_THREADPOOLS].execute(move || {
                common::check_prime(i);
            });
        }
    });
}
