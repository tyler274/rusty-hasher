#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use std::io;
use std::result;

use rusty_hasher::dictionary_words::{DICTIONARY, NUM_DICTIONARY_WORDS};
use rusty_hasher::thread_pool::{
    thread_pool_add_work, thread_pool_finish, thread_pool_init, thread_pool_t,
};

pub type work_function_t = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;

pub const HASH_START: &str = "$6$";

pub const SALT_LENGTH: usize = 20;

pub const HASH_LENGTH: usize = 106;

pub const NUM_THREADS: usize = 16;

#[inline]
fn hashes_match(password: &str, hash: &str) -> bool {
    pwhash::unix_crypt::verify(password, hash)
}

pub fn word_variants(word: &str) -> Vec<String> {
    let word_length = word.len();
    // Each word has a length W. there are 10 numbered character s (0..9) that could be
    // inserted anywhere in the string.
    // There are (W+1) places we can insert the number in the string.
    // There are (W+1)*10 variants of the word.
    // This doesn't count the null terminator, because it is always the last character.
    let word_variant_count = word_length.wrapping_add(1).wrapping_mul(10);
    // include the null terminator
    let word_length_with_null_term: libc::size_t = word_length.wrapping_add(1);

    let mut variants = vec!["".to_string(); word_variant_count];
    // i = 0 ... i = 9
    for i in 0..10 {
        // iterate through the non null character positions in the string.
        for j in 0..word_length + 1 {
            // TODO: is this [..j] or []..j+1]
            let mut result_word = std::str::from_utf8(&word.as_bytes()[..j])
                .unwrap()
                .to_string();
            let number = format!("{i}");
            result_word.insert_str(j, &number);

            result_word.insert_str(j + 1, std::str::from_utf8(&word.as_bytes()[j..]).unwrap());
            // printf("variant of \"%s\": \"%s\"\n", word, result_word);
            variants[(i * (word_length + 1)) + j] = result_word
        }
    }
    variants
}

pub fn check_word_variants(dictionary_word: &str, hashes: &Vec<String>) {
    let mut word = dictionary_word;
    let mut variants = word_variants(word);
    for variant in variants {
        for hash in hashes {
            if hashes_match(&variant, &hash) {
                println!("{}:{}", variant, hash);
            }
        }
    }
}
unsafe fn main_0() -> libc::c_int {
    // Read in the hashes from the standard input
    // let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    // let mut line_capacity: libc::size_t = 0;
    // Stop when the end of the input or an empty line is reached

    let hashes: Vec<String> = Vec::new();
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).unwrap() != 0 && buffer.as_bytes()[0] as char != '\n' {
        assert!(
            buffer.len() == HASH_LENGTH
                || buffer.len() == HASH_LENGTH.wrapping_add(1)
                    && buffer.as_bytes()[HASH_LENGTH] as char == '\n'
        );
        assert!(buffer == HASH_START);
        // Extend the hashes array and add the hash to it

        let hash = std::str::from_utf8(&buffer.as_bytes()[..HASH_LENGTH]).unwrap();
        hashes.push(hash.to_string());
    }

    // TODO (student): Use your threadpool to recover the passwords from the hashes.
    //                 You may assume the provided dictionary.h includes all the
    //                 possible root words.
    let mut pool: &mut thread_pool_t = thread_pool_init(30);

    for i in 0..NUM_DICTIONARY_WORDS {
        thread_pool_add_work(
            pool,
            Some(check_word_variants as fn(_: &str, _: &Vec<String>) -> ()),
            DICTIONARY[i],
        );
    }
    thread_pool_finish(pool);
    return 0;
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
