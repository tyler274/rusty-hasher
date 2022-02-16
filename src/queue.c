#include "queue.h"

#include <assert.h>
#include <pthread.h>
#include <stdlib.h>

struct queue {
    // queue length
    size_t len;
    bool in_use;

    // Head Node pointer
    queue_node_t *head;

    // Tail Node pointer
    queue_node_t *tail;

    // The multithreading canticles.
    pthread_mutex_t mutex;
    pthread_cond_t writing_cond;
    pthread_cond_t reading_cond;
    size_t watchers_reading;
};

struct queue_node {
    void *contents;
    queue_node_t *previous;
    queue_node_t *next;
};

// TODO (student): Write this!
/**
 * Creates a new heap-allocated FIFO queue. The queue is initially empty.
 *
 * @return a pointer to the new queue
 */
queue_t *queue_init(void) {
    queue_t *queue = (queue_t *) calloc(1, sizeof(queue_t));

    if (queue == NULL) {
        return NULL;
    }
    queue->in_use = true;
    queue->head = NULL;
    queue->tail = NULL;

    // initializes the mutex and conditions.
    int pthread_error = 0;
    pthread_error = pthread_mutex_init(&queue->mutex, NULL);
    assert(!pthread_error);
    pthread_error = pthread_cond_init(&queue->reading_cond, NULL);
    assert(!pthread_error);
    pthread_error = pthread_cond_init(&queue->writing_cond, NULL);
    assert(!pthread_error);

    // There are zero threads reading the queue at initialization.
    queue->watchers_reading = 0;

    return queue;
}

/**
 * Enqueues a value into a queue. There is no maximum capacity,
 * so this should succeed unless the program runs out of memory.
 * This function should be concurrency-safe:
 * multiple threads may call queue_enqueue() or queue_dequeue() simultaneously.
 *
 * @param queue the queue to append to
 * @param value the value to add to the back of the queue
 */
void queue_enqueue(queue_t *queue, void *value) {
}

/**
 * Dequeues a value from a queue.
 * The value returned is the first enqueued value that was not yet dequeued.
 * If the queue is empty, this thread should block until another thread enqueues a value.
 * This function should be concurrency-safe:
 * multiple threads may call queue_enqueue() or queue_dequeue() simultaneously.
 *
 * @param queue the queue to remove from
 * @return the value at the front of the queue
 */
void *queue_dequeue(queue_t *queue) {
}

/**
 * Frees all resources associated with a heap-allocated queue.
 * You may assume that the queue is already empty.
 *
 * @param queue a queue returned from queue_init()
 */
void queue_free(queue_t *queue) {
}