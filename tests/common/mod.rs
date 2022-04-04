pub fn usage(argv: &[String]) {
    eprintln!("Usage: {} <# of threads>", argv[0]);
    std::process::exit(1);
}

pub fn check_prime(n: u64) {
    let mut k: u64 = n.wrapping_div(2);
    while k > 1 {
        if n.wrapping_rem(k) == 0 {
            return;
        }
        k = k.wrapping_sub(1)
    }
    println!("{n}");
}

pub mod dictionary_words;
use dictionary_words::*;
