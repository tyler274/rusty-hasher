#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct queue_t {
    pub len: libc::size_t,
    pub in_use: bool,
    pub head: *mut queue_node_t,
    pub tail: *mut queue_node_t,
    pub mutex: libc::pthread_mutex_t,
    pub writing_cond: libc::pthread_cond_t,
    pub threads_reading: libc::size_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct queue_node_t {
    pub contents: *mut libc::c_void,
    pub previous: *mut queue_node_t,
    pub next: *mut queue_node_t,
}

/* *
 * Gets a writing lock.
 * Blocks on other threads that are reading the queue.
 *
 * @param queue the queue to get a write lock for.
 */
unsafe extern "C" fn get_write_lock_enqueue(mut queue: *mut queue_t) {
    // Gets a lock and then waits until the thread can enqueue an item.
    libc::pthread_mutex_lock(&mut (*queue).mutex);
    // Block until there are no threads reading the queue, or the queue is flagged as no
    // longer in use.
    while (*queue).in_use as libc::c_int != 0 && (*queue).threads_reading > 0 {
        libc::pthread_cond_wait(&mut (*queue).writing_cond, &mut (*queue).mutex);
    }
}
unsafe extern "C" fn get_write_lock_dequeue(mut queue: *mut queue_t) {
    libc::pthread_mutex_lock(&mut (*queue).mutex);
    // Block until there are values in the queue or the queue is flagged as no longer in
    // use.
    while (*queue).in_use as libc::c_int != 0
        && ((*queue).head.is_null() || (*queue).threads_reading > 0)
    {
        libc::pthread_cond_wait(&mut (*queue).writing_cond, &mut (*queue).mutex);
    }
}
/* *
 * Gives back a writing lock.
 *
 * @param queue the queue to get a write lock for.
 */
unsafe extern "C" fn release_write_lock(mut queue: *mut queue_t) {
    // Broadcast to everyone listening on the writing conditions that they may
    // have something to do.
    libc::pthread_cond_broadcast(&mut (*queue).writing_cond);
    // unlock the mutex.
    libc::pthread_mutex_unlock(&mut (*queue).mutex);
}
/* *
 * Creates a new heap-allocated FIFO queue. The queue is initially empty.
 *
 * @return a pointer to the new queue
 */
#[no_mangle]
pub unsafe extern "C" fn queue_init() -> *mut queue_t {
    let mut queue: *mut queue_t = libc::calloc(1, ::std::mem::size_of::<queue_t>()) as *mut queue_t;
    if queue.is_null() {
        return 0 as *mut queue_t;
    }
    (*queue).head = 0 as *mut queue_node_t;
    (*queue).tail = 0 as *mut queue_node_t;
    // initializes the mutex and conditions.
    let mut pthread_error: libc::c_int = 0 as libc::c_int;
    pthread_error =
        libc::pthread_mutex_init(&mut (*queue).mutex, 0 as *const libc::pthread_mutexattr_t);
    assert!(pthread_error == 0);
    pthread_error = libc::pthread_cond_init(
        &mut (*queue).writing_cond,
        0 as *const libc::pthread_condattr_t,
    );
    assert!(pthread_error == 0);
    // There are zero threads reading the queue at initialization.
    (*queue).threads_reading = 0 as libc::c_int as libc::size_t;
    // Set this last otherwise the threads get eager?
    (*queue).in_use = 1 as libc::c_int != 0;
    return queue;
}
/* *
 * @brief Initializes the head and tail nodes of the given queue
 *
 * Does nothing if the provided content pointer isn't null or the head and tail nodes
 * aren't NULL.
 *
 * @param queue
 * @param content_p
 *
 * @return whether there was an error initializing the head (and tail) nodes, which means
 * it was called for a non empty queue.
 */
unsafe extern "C" fn initialize_head_node(
    mut queue: *mut queue_t,
    mut content_p: *mut libc::c_void,
) -> libc::c_int {
    if queue.is_null() || !(*queue).head.is_null() || !(*queue).tail.is_null() {
        // Do nothing if this is called on a non-empty queue.
        // Return 1 to let the caller know this was called on a non-empty queue.
        return 1 as libc::c_int;
    }
    // initialize the head node and set its content pointer to the passed value.
    (*queue).head = libc::calloc(1, ::std::mem::size_of::<queue_node_t>()) as *mut queue_node_t;
    (*(*queue).head).contents = content_p;
    // The only node in the queue is this node, no previous or next to go to.
    (*(*queue).head).previous = 0 as *mut queue_node_t;
    (*(*queue).head).next = 0 as *mut queue_node_t;
    // set the tail node to be the head node as that is the initital configuration.
    (*queue).tail = (*queue).head;
    (*queue).len = 1 as libc::c_int as libc::size_t;
    // Return 0 as all is well.
    return 0 as libc::c_int;
}
/* *
 * Enqueues a value into a queue. There is no maximum capacity,
 * so this should succeed unless the program runs out of memory.
 * This function should be concurrency-safe:
 * multiple threads may call queue_enqueue() or queue_dequeue() simultaneously.
 *
 * @param queue the queue to append to
 * @param value the value to add to the back of the queue
 */

pub fn queue_enqueue<T: Sync>(queue: &mut queue_t, value: &mut T) {
    // If the queue is flagged for clean up or either of the arguments are NULL pointers
    // just return (for now).
    // Ok the first value queued by the multithreaded tester is 0 which evaluates as NULL
    // here and messes those up. So always queue something even if its a null/0 value.
    if queue.is_null() || !(*queue).in_use {
        // TODO: rewrite API to have some error handling/acknowledgement.
        return;
    }
    // get the write lock for enqueueing values.
    get_write_lock_enqueue(queue);
    if !(*queue).in_use {
        // TODO: error handling.
        return;
    } else {
        if (*queue).head.is_null() || (*queue).tail.is_null() {
            let mut i: libc::c_int = initialize_head_node(queue, value);
            assert!(i == 0);
        } else {
            let mut node_to_enqueue: *mut queue_node_t =
                libc::calloc(1, ::std::mem::size_of::<queue_node_t>()) as *mut queue_node_t;
            if node_to_enqueue.is_null() {
                // TODO: Error handling
                return;
            } else {
                // Set the node we are enqueueing's value/content.
                (*node_to_enqueue).contents = value;
                // Set the node we are enqueueing's previous node pointer to the queue's
                // current (and soon to be former) tail.
                (*node_to_enqueue).previous = (*queue).tail;
                // As this is a first in first out queue we are pushing to the back of the
                // queue, so there is no next node for this node to point to.
                (*node_to_enqueue).next = 0 as *mut queue_node_t;
                // Update the queue's soon to be old tail's next node pointer to point to the
                // node are enqueueing.
                (*(*queue).tail).next = node_to_enqueue;
                // Set the queue's tail to be the node we have now enqueued as the back of the
                // queue.
                (*queue).tail = node_to_enqueue;
                assert!((*queue).len.wrapping_add(1) < 18446744073709551615);
                (*queue).len = ((*queue).len).wrapping_add(1)
            }
        }
    }
    // Release the lock we took to write to the queue.
    release_write_lock(queue);
}
/* *
 * Dequeues a value from a queue.
 * The value returned is the first enqueued value that was not yet dequeued.
 * If the queue is empty, this thread should block until another thread enqueues a value.
 * This function should be concurrency-safe:
 * multiple threads may call queue_enqueue() or queue_dequeue() simultaneously.
 *
 * @param queue the queue to remove from
 * @return the value at the front of the queue
 */

pub fn queue_dequeue(queue: &mut queue_t) -> impl Sync {
    if queue.is_null() || !(*queue).in_use {
        return 0 as *mut libc::c_void;
    }
    let mut head_value: *mut libc::c_void = 0 as *mut libc::c_void;
    // Get a writing lock approppriate for taking something out of the queue.
    get_write_lock_dequeue(queue);
    // check that the queue is still in use.
    if (*queue).in_use {
        head_value = (*(*queue).head).contents;
        if (*queue).head == (*queue).tail {
            // If head and tail are the same we de-initialize both after freeing the node.
            libc::free((*queue).head as *mut libc::c_void);
            (*queue).head = 0 as *mut queue_node_t;
            (*queue).tail = 0 as *mut queue_node_t;
            (*queue).len = 0 as libc::c_int as libc::size_t
        } else {
            // Get the next node in the queeu, and free the prior head, and set the new
            // node as the head.
            let mut next_node: *mut queue_node_t = (*(*queue).head).next;
            libc::free((*queue).head as *mut libc::c_void);
            (*next_node).previous = 0 as *mut queue_node_t;
            (*queue).head = next_node;
            assert!((*queue).len.wrapping_sub(1) >= 0);
            (*queue).len = ((*queue).len).wrapping_sub(1)
        }
    }
    // Release the write lock we took.
    release_write_lock(queue);
    return head_value;
}
/* *
 * Frees all resources associated with a heap-allocated queue.
 * You may assume that the queue is already empty.
 *
 * @param queue a queue returned from queue_init()
 */
#[no_mangle]
pub unsafe extern "C" fn queue_free(mut queue: *mut queue_t) {
    if queue.is_null() || !(*queue).in_use {
        return;
    }
    get_write_lock_enqueue(queue);
    if (*queue).in_use {
        let mut next_node: *mut queue_node_t = 0 as *mut queue_node_t;
        let mut current_node: *mut queue_node_t = (*queue).head;
        while !current_node.is_null() {
            next_node = (*current_node).next;
            libc::free(current_node as *mut libc::c_void);
            current_node = next_node
        }
        (*queue).head = 0 as *mut queue_node_t;
        (*queue).tail = 0 as *mut queue_node_t;
        (*queue).in_use = 0 as libc::c_int != 0
    }
    release_write_lock(queue);
    assert!((*queue).threads_reading == 0);
    // free the pthread items
    // TODO: make sure this doesnt footgun threads
    libc::pthread_mutex_destroy(&mut (*queue).mutex);
    libc::pthread_cond_destroy(&mut (*queue).writing_cond);
    // free the heap allocated queue itself.
    libc::free(queue as *mut libc::c_void);
}
