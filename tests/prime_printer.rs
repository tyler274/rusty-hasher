#![feature(scoped_threads)]
mod common;
use scoped_threadpool::ThreadPool;

pub const MAX_CANDIDATE: u64 = 100000;
const NUM_THREADS_PER_POOL: usize = 8;

#[test]
pub fn prime_printer() {
    // let args: Vec<String> = std::env::args().collect();
    // if args.len() != 2 {
    //     common::usage(&args);
    // }
    // let num_threads = args[1].parse::<usize>().unwrap();
    // if num_threads == 0 {
    //     common::usage(&args);
    // }
    std::thread::scope(|scope| {
        let pool = ThreadPool::new(NUM_THREADS_PER_POOL, scope);
        for i in 2..MAX_CANDIDATE {
            pool.execute(move || {
                common::check_prime(i);
            });
        }
    });
}
