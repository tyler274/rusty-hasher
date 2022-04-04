#![feature(scoped_threads)]
use std::{thread, time::Duration};

use rusty_hasher::thread_pool::ThreadPool;

pub const NS_PER_SEC: Duration = Duration::from_nanos(1_000_000_000);

pub const NUM_SLEEPS: usize = 10;

pub fn usage(args: &[String]) {
    eprintln!("Usage: {} <# of threads>", args[2]);
    std::process::exit(1);
}

pub fn sleeper(dur: Duration) {
    thread::sleep(dur);
}
pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args[1] != "2" {
        usage(&args);
    }
    let num_threads = args[1].parse::<usize>().unwrap();
    if num_threads == 0 {
        usage(&args);
    }
    let result = std::time::Instant::now();
    thread::scope(|scope| {
        let pool = ThreadPool::new(num_threads, scope);
        for _ in 0..NUM_SLEEPS {
            pool.execute(|| sleeper(Duration::from_nanos(0)));
        }
    });

    let time_passed = result.elapsed();
    println!("{:#?}", time_passed);
    std::process::exit(0);
}
