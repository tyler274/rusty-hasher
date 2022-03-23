#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::queue::{queue_dequeue, queue_enqueue, queue_free, queue_init, queue_t};

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Thunk<'a> = Box<dyn FnBox + Send + 'a>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct thread_pool_t {
    pub threads: *mut libc::pthread_t,
    pub num_threads: libc::size_t,
    pub work_queue: *mut queue_t,
}

pub type work_function_t = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct task_t {
    pub function: work_function_t,
    pub aux: *mut libc::c_void,
}

// pthread_cond_t work_in_queue_cond;
// bool in_use;
#[no_mangle]
pub extern "C" fn get_work_from_queue(mut pool: *mut libc::c_void) -> *mut libc::c_void {
    unsafe {
        let mut thread_pool: *mut thread_pool_t = pool as *mut thread_pool_t;
        let mut task: *mut task_t = 0 as *mut task_t;
        task = queue_dequeue((*thread_pool).work_queue) as *mut task_t;
        while !task.is_null() {
            assert!(!task.is_null());
            assert!((*task).function.is_some());
            (*task).function.expect("non-null function pointer")((*task).aux);
            libc::free(task as *mut libc::c_void);
            task = queue_dequeue((*thread_pool).work_queue) as *mut task_t
        }
        return 0 as *mut libc::c_void;
    }
}
/* *
 * Creates a new heap-allocated thread pool with the given number of worker threads.
 * All worker threads should start immediately so they can perform work
 * as soon as thread_pool_add_work() is called.
 *
 * @param num_worker_threads the number of threads in the pool
 * @return a pointer to the new thread pool
 */
#[no_mangle]
pub unsafe extern "C" fn thread_pool_init(
    mut num_worker_threads: libc::size_t,
) -> *mut thread_pool_t {
    let mut thread_pool: *mut thread_pool_t =
        libc::calloc(1, ::std::mem::size_of::<thread_pool_t>()) as *mut thread_pool_t;
    if thread_pool.is_null() {
        return 0 as *mut thread_pool_t;
    }
    (*thread_pool).num_threads = num_worker_threads;
    (*thread_pool).work_queue = queue_init();
    (*thread_pool).threads = libc::calloc(
        (*thread_pool).num_threads,
        ::std::mem::size_of::<libc::pthread_t>(),
    ) as *mut libc::pthread_t;
    if (*thread_pool).threads.is_null() {
        return 0 as *mut thread_pool_t;
    }
    let mut i: libc::size_t = 0 as libc::c_int as libc::size_t;
    while i < (*thread_pool).num_threads {
        // queue_enqueue(thread_pool->work_queue, NULL);
        let mut pthread_error: libc::c_int = libc::pthread_create(
            &mut *(*thread_pool).threads.offset(i as isize),
            0 as *const libc::pthread_attr_t,
            get_work_from_queue as extern "C" fn(_: *mut libc::c_void) -> *mut libc::c_void,
            thread_pool as *mut libc::c_void,
        );
        assert!(pthread_error == 0);
        i = i.wrapping_add(1)
    }
    // pthread_cond_init(&thread_pool->work_in_queue_cond, NULL);
    // thread_pool->in_use = true;
    return thread_pool;
}
/* *
 * Adds work to a thread pool.
 * The work will be performed by a worker thread as soon as all previous work is finished.
 *
 * @param pool the thread pool to perform the work
 * @param function the function to call on a thread in the thread pool
 * @param aux the argument to call the work function with
 */

pub fn thread_pool_add_work(
    mut pool: *mut thread_pool_t,
    mut function: work_function_t,
    mut aux: *mut libc::c_void,
) {
    // let mut task: *mut task_t = libc::calloc(1, ::std::mem::size_of::<task_t>()) as
    // *mut task_t;
    let mut task: Box<task_t> = Box::new(task_t { function, aux });
    queue_enqueue(pool.deref().work_queue, task);
}
/* *
 * Waits for all work added to a thread pool to finish,
 * then frees all resources associated with a heap-allocated thread pool.
 * A special value (e.g. NULL) can be put in the work queue to mark the end of the work.
 * thread_pool_add_work() cannot be used on this pool once this function is called.
 *
 * @param pool the thread pool to close
 */
#[no_mangle]
pub unsafe extern "C" fn thread_pool_finish(mut pool: *mut thread_pool_t) {
    let mut i: libc::size_t = 0 as libc::c_int as libc::size_t;
    while i < (*pool).num_threads {
        queue_enqueue(pool.deref().work_queue, 0 as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    let mut i_0: libc::size_t = 0 as libc::c_int as libc::size_t;
    while i_0 < (*pool).num_threads {
        libc::pthread_join(
            *(*pool).threads.offset(i_0 as isize),
            0 as *mut *mut libc::c_void,
        );
        i_0 = i_0.wrapping_add(1)
    }
    libc::free((*pool).threads as *mut libc::c_void);
    queue_free((*pool).work_queue);
    libc::free(pool as *mut libc::c_void);
}
