use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// https://doc.rust-lang.org/book/ch20-02-multithreaded.html
pub struct ThreadPool<'scope, 'env> {
    pub workers: Vec<Worker<'scope>>,
    sender: mpsc::Sender<Job<'env>>,
}

// impl<'scope, 'env> Clone for ThreadPool<'scope, 'env> {
//     fn clone(&self) -> Self {
//         Self {
//             workers: self.workers.clone(),
//             sender: self.sender.clone(),
//         }
//     }
// }

type Job<'env> = Box<dyn FnOnce() + Send + 'env>;

impl<'scope, 'env> ThreadPool<'scope, 'env> {
    /* *
     * Creates a new thread pool with the given number of worker threads.
     * All worker threads should start immediately so they can perform work
     * as soon as self.execute() is called.
     *
     * @param num_worker_threads the number of threads in the pool
     * @param scope the thread pool's variable lifetime scope.
     * @return the new thread pool
     * Lifetimes 'scope and 'env are documented here
     * https://doc.rust-lang.org/nightly/std/thread/fn.scope.html#lifetimes
     */
    pub fn new(
        num_worker_threads: usize,
        scope: &'scope thread::Scope<'scope, 'env>,
    ) -> ThreadPool<'scope, 'env> {
        assert!(num_worker_threads > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(num_worker_threads);

        for id in 0..num_worker_threads {
            workers.push(Worker::new(id, scope, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }
    /* *
     * Adds work to a thread pool.
     * The work will be performed by a worker thread as soon as all previous work is finished.
     *
     * @param f the function to call on a thread in the thread pool
     * Note the lifetime 'env on function f in accordance to
     * https://doc.rust-lang.org/nightly/std/thread/fn.scope.html#lifetimes
     */
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'env,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

pub struct Worker<'scope> {
    pub id: usize,
    pub thread: thread::ScopedJoinHandle<'scope, ()>,
}

impl<'scope, 'env> Worker<'scope> {
    /* *
     * Worker that processes jobs added to the threadpool as they are added.
     *
     * @param id the worker (thread's) identifying number.
     * @param scope the variable lifetime scope to use.
     * @param receiver is the queue that is waited on for new jobs to execute.
     * @return the the worker
     */
    fn new(
        id: usize,
        scope: &'scope thread::Scope<'scope, 'env>,
        receiver: Arc<Mutex<mpsc::Receiver<Job<'env>>>>,
    ) -> Worker<'scope> {
        let thread = scope.spawn(move || loop {
            let job = {
                let lock = receiver
                    .lock()
                    .expect("Worker thread unable to lock the job receiver");
                lock.recv()
            };

            // println!("Worker {} got a job; executing.", id);

            let job = match job {
                Ok(job) => job,
                // ThreadPool has been dropped
                Err(..) => break,
            };

            job();

            // println!("Worker {} completed its job; executing.", id);
        });

        Worker { id, thread }
    }
}
