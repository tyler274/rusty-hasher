#define _GNU_SOURCE
#include <assert.h>
#include <crypt.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "dictionary_words.h"
#include "thread_pool.h"

const char HASH_START[] = "$6$";
const size_t SALT_LENGTH = 20;
const size_t HASH_LENGTH = 106;
const size_t NUM_THREADS = 16;

static size_t hash_count = 0;
static char **hashes = NULL;

// typedef void *(*GetItemFunc)(struct array_t *list, size_t index);

// typedef struct array {
//     size_t len;
//     size_t item_size;
//     void *items;

//     // size_t (*Length)(array_t *);
//     GetItemFunc get;

// } array_t;

// void *GetItemImpl(array_t *self, size_t index) {
//     void *item = (void *) (self->items + index * self->item_size);
// }

// typedef array_t (*word_variants)(struct string_t *word);

// typedef struct string {
//     size_t len;
//     char *contents;

//     word_variants variants;
// } string_t;

// string_t *new_string(char *word) {
//     if (word == NULL) {
//         return NULL;
//     }
//     string_t *ns = calloc(1, sizeof(string_t));
//     size_t word_length = 0;
//     for (size_t i = 0; word[i] != '\x00'; i++) {
//         ns->len = i;
//     }
//     ns->len += 1;

//     ns->contents = (char *) calloc(ns->len, sizeof(char));
//     assert(ns->contents != NULL);
//     memcpy(ns->contents, word, ns->len * sizeof(char));

//     ns->variants = word_variants_impl;

//     return ns;
// }

// array_t word_variants_impl(string_t *word) {

// }

static inline bool hashes_match(const char *password, const char *hash) {
    char salt[SALT_LENGTH + 1];
    memcpy(salt, hash, sizeof(char[SALT_LENGTH]));
    salt[SALT_LENGTH] = '\0';
    struct crypt_data data;
    memset(&data, 0, sizeof(data));
    char *hashed = crypt_r(password, salt, &data);
    char *hashed_hash = &hashed[SALT_LENGTH];
    const char *hash_hash = &hash[SALT_LENGTH];
    return memcmp(hashed_hash, hash_hash, sizeof(char[HASH_LENGTH - SALT_LENGTH])) == 0;
}

typedef struct word_array {
    size_t len;
    size_t word_size;
    char **words;
} word_array_t;

word_array_t word_variants(char *word) {
    size_t word_length = strlen(word);
    // Each word has a length W. there are 10 numbered character s (0..9) that could be
    // inserted anywhere in the string.
    // There are (W+1) places we can insert the number in the string.
    // There are (W+1)*10 variants of the word.
    // This doesn't count the null terminator, because it is always the last character.
    size_t word_variant_count = (word_length + 1) * 10;

    // include the null terminator
    size_t word_length_with_null_term = word_length + 1;

    char **variants = calloc(word_variant_count, sizeof(char *));
    // 0..9
    for (size_t i = 0; i < 10; i++) {
        // iterate through the non null character positions in the string.
        for (size_t j = 0; j <= word_length; j++) {
            char *result_word = calloc(word_length_with_null_term + 1, sizeof(char));
            strncpy(result_word, word, j);
            char number[2];
            sprintf(number, "%lu", i);
            strncpy(&result_word[j], number, 1);
            strncpy(&result_word[j + 1], &word[j], word_length - j);
            // printf("variant of \"%s\": \"%s\"\n", word, result_word);
            variants[(i * (word_length + 1)) + j] = result_word;
        }
    }

    word_array_t variants_array = {.len = word_variant_count,
                                   .word_size = word_length_with_null_term + 1,
                                   .words = variants};

    return variants_array;
}

void free_variants(word_array_t variants) {
    for (size_t i = 0; i < variants.len; i++) {
        free(variants.words[i]);
    }
    free(variants.words);
}

void check_word_variants(void *dictionary_word) {
    char *word = (char *) dictionary_word;
    word_array_t variants = word_variants(word);
    for (size_t i = 0; i < variants.len; i++) {
        for (size_t j = 0; j < hash_count; j++) {
            if (hashes_match(variants.words[i], hashes[j])) {
                printf("%s:%s\n", variants.words[i], hashes[j]);
            }
        }
    }

    free_variants(variants);
    return;
}

int main(void) {
    // Read in the hashes from the standard input
    char *line = NULL;
    size_t line_capacity = 0;

    // Stop when the end of the input or an empty line is reached
    while (getline(&line, &line_capacity, stdin) > 0 && line[0] != '\n') {
        // Check that the line looks like a hash
        size_t line_length = strlen(line);
        assert(line_length == HASH_LENGTH ||
               (line_length == HASH_LENGTH + 1 && line[HASH_LENGTH] == '\n'));
        assert(memcmp(line, HASH_START, sizeof(HASH_START) - sizeof(char)) == 0);

        // Extend the hashes array and add the hash to it
        hashes = realloc(hashes, sizeof(char * [hash_count + 1]));
        assert(hashes != NULL);
        char *hash = malloc(sizeof(char[HASH_LENGTH + 1]));
        assert(hash != NULL);
        memcpy(hash, line, sizeof(char[HASH_LENGTH]));
        hash[HASH_LENGTH] = '\0';
        hashes[hash_count++] = hash;
        // printf("Hash added: \"%s\", index: %lu\n", hashes[hash_count - 1],
        //        hash_count - 1);
    }
    free(line);

    // TODO (student): Use your threadpool to recover the passwords from the hashes.
    //                 You may assume the provided dictionary.h includes all the
    //                 possible root words.
    thread_pool_t *pool = thread_pool_init(30);

    for (size_t i = 0; i < NUM_DICTIONARY_WORDS; i++) {
        thread_pool_add_work(pool, check_word_variants, (void *) DICTIONARY[i]);
    }

    thread_pool_finish(pool);
}
