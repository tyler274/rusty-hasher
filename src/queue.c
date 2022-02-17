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
    // Oh Omnissiah bless this Producer Consumer, this pub sub, this dinner of your most
    // faithful.
    pthread_mutex_t mutex;
    pthread_cond_t writing_cond;
    pthread_cond_t reading_cond;
    size_t threads_reading;
};

struct queue_node {
    void *contents;
    queue_node_t *previous;
    queue_node_t *next;
};

/**
 * Gets a writing lock.
 * Blocks on other threads that are reading the queue.
 *
 * @param queue the queue to get a write lock for.
 */
static void get_write_lock_enqueue(queue_t *queue) {
    // Gets a lock and then waits until the thread can enqueue an item.
    pthread_mutex_lock(&queue->mutex);

    // Block until there are no threads reading the queue, or the queue is flagged as no
    // longer in use.
    while (queue->in_use && queue->threads_reading > 0) {
        pthread_cond_wait(&queue->writing_cond, &queue->mutex);
    }
}

static void get_write_lock_dequeue(queue_t *queue) {
    pthread_mutex_lock(&queue->mutex);
    // Block until there are values in the queue or the queue is flagged as no longer in
    // use.
    while (queue->in_use && (queue->head == NULL || queue->threads_reading > 0)) {
        pthread_cond_wait(&queue->writing_cond, &queue->mutex);
    }
}

/**
 * Gives back a writing lock.
 *
 * @param queue the queue to get a write lock for.
 */
static void release_write_lock(queue_t *queue) {
    // Broadcast to everyone listening on the reading and writing conditions that they may
    // have something to do.
    pthread_cond_broadcast(&queue->reading_cond);
    pthread_cond_broadcast(&queue->writing_cond);
    // unlock the mutex.
    pthread_mutex_unlock(&queue->mutex);
}

/**
 * @brief Gets a reading lock.
 *
 * @param queue the queue to get a read lock for.
 */
static void get_read_lock(queue_t *queue) {
    // yoink our lock from the mutex.
    pthread_mutex_lock(&queue->mutex);
    // wait on the read condition if there queue's head is NULL (queue is empty) or until
    // the queue is flagged as no longer in use.
    while (queue->in_use && queue->head == NULL) {
        pthread_cond_wait(&queue->reading_cond, &queue->mutex);
    }
    // Increement the count of threads reading the queue
    queue->threads_reading += 1;
    // give back the lock we took at the start.
    pthread_mutex_unlock(&queue->mutex);
}

/**
 * Gives back a reading lock.
 *
 * @param queue the queue to return the read lock for.
 */
static void release_read_lock(queue_t *queue) {
    // lock the queue's mutex for this notification.
    pthread_mutex_lock(&queue->mutex);
    // decrement the count of the number of threads reading the queue.
    if (queue->threads_reading - 1 >= 0) {
        queue->threads_reading -= 1;
    }

    assert(queue->threads_reading >= 0);

    // If there are no other threads reading, just broadcast to the threads waiting
    // for a write lock as any thread wanting to read will need to get a lock anyway.
    if (queue->threads_reading == 0) {
        pthread_cond_broadcast(&queue->reading_cond);
        pthread_cond_broadcast(&queue->writing_cond);
    }
    // release the lock we took at the start.
    pthread_mutex_unlock(&queue->mutex);
}

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
    queue->threads_reading = 0;

    // Set this last otherwise the threads get eager?
    queue->in_use = true;

    return queue;
}

/**
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
static int initialize_head_node(queue_t *queue, void *content_p) {
    if (queue == NULL /*|| content_p == NULL*/ || queue->head != NULL ||
        queue->tail != NULL) {
        // Do nothing if this is called on a non-empty queue.
        // Return 1 to let the caller know this was called on a non-empty queue.
        return 1;
    }
    // initialize the head node and set its content pointer to the passed value.
    queue->head = (queue_node_t *) calloc(1, sizeof(queue_node_t));
    queue->head->contents = content_p;
    // The only node in the queue is this node, no previous or next to go to.
    queue->head->previous = NULL;
    queue->head->next = NULL;
    // set the tail node to be the head node as that is the initital configuration.
    queue->tail = queue->head;
    queue->len = 1;

    // Return 0 as all is well.
    return 0;
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
    // If the queue is flagged for clean up or either of the arguments are NULL pointers
    // just return (for now).
    // Ok the first value queued by the multithreaded tester is 0 which evaluates as NULL
    // here and messes those up. So always queue something even if its a null/0 value.
    if (queue == NULL /*|| value == NULL*/ || !queue->in_use) {
        // TODO: rewrite API to have some error handling/acknowledgement.
        return;
    }

    // get the write lock for enqueueing values.
    get_write_lock_enqueue(queue);
    if (!queue->in_use) {
        // TODO: error handling.
        return;
    }
    else if (queue->head == NULL || queue->tail == NULL) {
        int i = initialize_head_node(queue, value);
        assert(i == 0);
    }
    else {
        queue_node_t *node_to_enqueue = (queue_node_t *) calloc(1, sizeof(queue_node_t));
        if (node_to_enqueue == NULL) {
            // TODO: Error handling
            return;
        }
        else {
            // Set the node we are enqueueing's value/content.
            node_to_enqueue->contents = value;
            // Set the node we are enqueueing's previous node pointer to the queue's
            // current (and soon to be former) tail.
            node_to_enqueue->previous = queue->tail;
            // As this is a first in first out queue we are pushing to the back of the
            // queue, so there is no next node for this node to point to.
            node_to_enqueue->next = NULL;

            // Update the queue's soon to be old tail's next node pointer to point to the
            // node are enqueueing.
            queue->tail->next = node_to_enqueue;
            // Set the queue's tail to be the node we have now enqueued as the back of the
            // queue.
            queue->tail = node_to_enqueue;

            assert(queue->len + 1 < __SIZE_MAX__);
            queue->len += 1;
        }
    }
    // Release the lock we took to write to the queue.
    release_write_lock(queue);
    return;
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
    if (queue == NULL || !queue->in_use) {
        return NULL;
    }

    void *head_value = NULL;
    // Get a writing lock approppriate for taking something out of the queue.
    get_write_lock_dequeue(queue);
    // check that the queue is still in use.
    if (queue->in_use) {
        head_value = queue->head->contents;

        if (queue->head == queue->tail) {
            // If head and tail are the same we de-initialize both after freeing the node.
            free(queue->head);
            queue->head = NULL;
            queue->tail = NULL;
            queue->len = 0;
        }
        else {
            // Get the next node in the queeu, and free the prior head, and set the new
            // node as the head.
            queue_node_t *next_node = queue->head->next;
            free(queue->head);
            next_node->previous = NULL;
            queue->head = next_node;
            assert(queue->len - 1 >= 0);
            queue->len -= 1;
        }
    }
    // Release the write lock we took.
    release_write_lock(queue);

    return head_value;
}

/**
 * Frees all resources associated with a heap-allocated queue.
 * You may assume that the queue is already empty.
 *
 * @param queue a queue returned from queue_init()
 */
void queue_free(queue_t *queue) {
    if (queue == NULL || !queue->in_use) {
        return;
    }

    get_write_lock_enqueue(queue);
    if (queue->in_use) {
        queue_node_t *next_node;

        for (queue_node_t *current_node = queue->head;
             current_node != NULL /*&& current_node <= queue->tail*/;
             current_node = next_node) {
            next_node = current_node->next;
            free(current_node);
        }

        queue->head = NULL;
        queue->tail = NULL;

        queue->in_use = false;
    }
    release_write_lock(queue);

    assert(queue->threads_reading == 0);
    // free the pthread items
    // TODO: make sure this doesnt footgun threads
    pthread_mutex_destroy(&queue->mutex);
    pthread_cond_destroy(&queue->reading_cond);
    pthread_cond_destroy(&queue->writing_cond);
    // free the heap allocated queue itself.
    free(queue);
}