#![feature(scoped_threads)]

use std::io;

use pwhash::sha512_crypt;
use scoped_threadpool_std::dictionary_words::DICTIONARY;
use scoped_threadpool_std::ThreadPool;

pub const HASH_START: &str = "$6$";

pub const SALT_LENGTH: usize = "$6$uoJHXvmYbiRlMqQe$".len();

pub const HASH_LENGTH: usize = "$6$uoJHXvmYbiRlMqQe$hN.9MB3XeewiWl86wacGEHdJDezR8ZLV0ttyL26KQDPQ6gt97sRvOzIIDrYADbsJhDP4z9TV2JnnPxy4DDYhm.".len();

pub const NUM_THREADS: usize = 30;

#[inline]
fn hashes_match(password: &str, hash: &str) -> bool {
    sha512_crypt::verify(password, hash)
}

pub fn word_variants(word: &str) -> Vec<String> {
    // Each word has a length W. there are 10 numbered character s (0..9) that could be
    // inserted anywhere in the string.
    // There are (W+1) places we can insert the number in the string.
    // There are (W+1)*10 variants of the word.
    // This doesn't count the null terminator, because it is always the last character.
    let word_variant_count = word.len().wrapping_add(1).wrapping_mul(10);

    let mut variants = vec!["".to_string(); word_variant_count];
    // i = 0 ... i = 9
    for i in 0..10 {
        // iterate through the non null character positions in the string.
        let number = format!("{i}");
        for (j, _k) in word.char_indices() {
            // eprintln!("Word: \"{word}\"");
            // eprintln!("Splitting at {:#?}", j);

            let (first_half, second_half) = word.split_at(j);
            let mut first_half = first_half.to_string();
            // eprintln!(
            //     "First half: \"{first_half}\", Second half: \"{second_half}\", Number: {number}"
            // );
            first_half.insert_str(j, &number);
            // eprintln!("First Half of Variant: \"{first_half}\"");
            first_half.insert_str(j + 1, &second_half);
            // eprintln!("Variant: {first_half}");
            variants[(i * (word.len() + 1)) + j] = first_half
        }
        // eprintln!("Word: \"{word}\"");
        // eprintln!("Splitting at {:#?}", word.len());

        // let (first_half, second_half) = word.split_at(j);
        // let mut first_half = first_half.to_string();
        // eprintln!("First half: \"{word}\", Second half: \"\", Number: {number}");
        // first_half.insert_str(j, &number);
        // eprintln!("First Half of Variant: \"{first_half}\"");
        // first_half.insert_str(j + 1, &second_half);
        // eprintln!("Variant: {first_half}");
        let mut last_variant = word.to_string();
        last_variant.insert_str(word.len(), &number);
        // eprintln!("Variant: {last_variant}");
        variants[(i * (word.len() + 1)) + word.len()] = last_variant;
    }
    variants
}

pub fn check_word_variants<'a>(dictionary_word: &str, hashes: &'a [String]) {
    let word = dictionary_word;
    let variants = word_variants(word);
    for variant in variants {
        for hash in hashes {
            if hashes_match(&variant, &hash) {
                println!("{}:{}", variant, hash);
            }
        }
    }
}
fn main() -> io::Result<()> {
    // Read in the hashes from the standard input
    // Stop when the end of the input or an empty line is reached
    let mut hashes: Vec<String> = Vec::new();
    // let dictionary = Box::new(get_dictionary());
    // println!("{dictionary:#?}");
    let mut buffer = String::with_capacity(HASH_LENGTH);
    while io::stdin().read_line(&mut buffer)? > 0 && buffer.as_bytes()[0] as char != '\n' {
        eprintln!(
            "HASH_LENGTH: {HASH_LENGTH}\nRead Buffer Length: {}\nBuffer: {buffer:#?}",
            buffer.len(),
        );
        assert!(
            buffer.len() == HASH_LENGTH
                || buffer.len() == HASH_LENGTH.wrapping_add(1)
                    && buffer.as_bytes()[HASH_LENGTH] as char == '\n'
        );
        assert!(std::str::from_utf8(&buffer.as_bytes()[..3]).unwrap() == HASH_START);
        // Extend the hashes array and add the hash to it

        let hash = std::str::from_utf8(&buffer.as_bytes()[..HASH_LENGTH]).unwrap();
        eprintln!("Extracted Hash: {hash:#?}");
        hashes.push(hash.to_string());
        buffer = String::with_capacity(HASH_LENGTH);
    }

    eprintln!("Finished loading hashes, processing...");
    std::thread::scope(|scope| {
        // TODO (student): Use your threadpool to recover the passwords from the hashes.
        //                 You may assume the provided dictionary.h includes all the
        //                 possible root words.
        let pool = ThreadPool::new(NUM_THREADS, scope);

        for word in DICTIONARY {
            //  Modeling on the API illustrated in
            //  https://doc.rust-lang.org/book/ch20-02-multithreaded.html
            pool.execute(|| {
                check_word_variants(word, &hashes);
            })
        }
    });

    std::process::exit(0)
}
