#include "thread_pool.h"

#include <assert.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdlib.h>

struct task {
    work_function_t function;
    void *aux;
};

struct thread_pool {
    pthread_t *threads;
    size_t num_threads;

    queue_t *work_queue;
    pthread_cond_t work_in_queue_cond;

    bool in_use;
};

void *get_work_from_queue(thread_pool_t *thread_pool) {
    task_t *task = NULL;
    for (task = (task_t *) queue_dequeue(thread_pool->work_queue); task != NULL;
         task = (task_t *) queue_dequeue(thread_pool->work_queue)) {
        assert(task != NULL);
        assert(task->function != NULL);
        task->function(task->aux);
        free(task);
    }
    return NULL;
}

/**
 * Creates a new heap-allocated thread pool with the given number of worker threads.
 * All worker threads should start immediately so they can perform work
 * as soon as thread_pool_add_work() is called.
 *
 * @param num_worker_threads the number of threads in the pool
 * @return a pointer to the new thread pool
 */
thread_pool_t *thread_pool_init(size_t num_worker_threads) {
    thread_pool_t *thread_pool = (thread_pool_t *) calloc(1, sizeof(thread_pool_t));
    if (thread_pool == NULL) {
        return NULL;
    }

    thread_pool->num_threads = num_worker_threads;
    thread_pool->work_queue = queue_init();

    thread_pool->threads =
        (pthread_t *) calloc(thread_pool->num_threads, sizeof(pthread_t));
    if (thread_pool->threads == NULL) {
        return NULL;
    }

    for (size_t i = 0; i < thread_pool->num_threads; i++) {
        // queue_enqueue(thread_pool->work_queue, NULL);
        int pthread_error = pthread_create(&thread_pool->threads[i], NULL,
                                           get_work_from_queue, thread_pool);
        assert(pthread_error == 0);
    }

    pthread_cond_init(&thread_pool->work_in_queue_cond, NULL);

    thread_pool->in_use = true;

    return thread_pool;
}

/**
 * Adds work to a thread pool.
 * The work will be performed by a worker thread as soon as all previous work is finished.
 *
 * @param pool the thread pool to perform the work
 * @param function the function to call on a thread in the thread pool
 * @param aux the argument to call the work function with
 */
void thread_pool_add_work(thread_pool_t *pool, work_function_t function, void *aux) {
    task_t *task = calloc(1, sizeof(task_t));
    assert(task != NULL);
    task->function = function;
    task->aux = aux;
    assert(function == 0x4ca930);
    queue_enqueue(pool->work_queue, task);
}

/**
 * Waits for all work added to a thread pool to finish,
 * then frees all resources associated with a heap-allocated thread pool.
 * A special value (e.g. NULL) can be put in the work queue to mark the end of the work.
 * thread_pool_add_work() cannot be used on this pool once this function is called.
 *
 * @param pool the thread pool to close
 */
void thread_pool_finish(thread_pool_t *pool) {
    for (size_t i = 0; i < pool->num_threads; i++) {
        queue_enqueue(pool->work_queue, NULL);
    }
    for (size_t i = 0; i < pool->num_threads; i++) {
        pthread_join(pool->threads[i], NULL);
    }

    free(pool->threads);

    queue_free(pool->work_queue);

    free(pool);
}