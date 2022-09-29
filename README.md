This provides a scoped threadpool utilizing the ~~nightly~~ stable scoped threads feature
https://github.com/rust-lang/rust/issues/93203

Test cases are provided including a sha512 password hasher to deduce passwords from the
provided dataset.

Documentation isnt up to snuff yet, and a lot of the old code needs to be cleaned out. 
