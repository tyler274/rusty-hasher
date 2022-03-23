#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type thread_pool;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn thread_pool_init(num_worker_threads: size_t) -> *mut thread_pool_t;
    #[no_mangle]
    fn thread_pool_add_work(
        pool: *mut thread_pool_t,
        function: work_function_t,
        aux: *mut libc::c_void,
    );
    #[no_mangle]
    fn thread_pool_finish(pool: *mut thread_pool_t);
}
pub type __uint64_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
pub type thread_pool_t = thread_pool;
pub type work_function_t = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[no_mangle]
pub static mut MAX_CANDIDATE: uint64_t = 100000 as libc::c_int as uint64_t;
#[no_mangle]
pub static mut NUM_THREADS_PER_POOL: size_t = 8 as libc::c_int as size_t;
#[no_mangle]
pub static mut NUM_THREADPOOLS: size_t = 10 as libc::c_int as size_t;
#[no_mangle]
pub unsafe extern "C" fn check_prime(mut _n: *mut libc::c_void) {
    let mut n: uint64_t = *(_n as *mut uint64_t);
    free(_n);
    let mut k: uint64_t = n.wrapping_div(2 as libc::c_int as libc::c_ulong);
    while k > 1 as libc::c_int as libc::c_ulong {
        if n.wrapping_rem(k) == 0 as libc::c_int as libc::c_ulong {
            return;
        }
        k = k.wrapping_sub(1)
    }
    printf(b"%lu\n\x00" as *const u8 as *const libc::c_char, n);
}
unsafe fn main_0() -> libc::c_int {
    let vla = NUM_THREADPOOLS as usize;
    let mut pools: Vec<*mut thread_pool_t> = ::std::vec::from_elem(0 as *mut thread_pool_t, vla);
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < NUM_THREADPOOLS {
        let ref mut fresh0 = *pools.as_mut_ptr().offset(i as isize);
        *fresh0 = thread_pool_init(NUM_THREADS_PER_POOL);
        i = i.wrapping_add(1)
    }
    let mut i_0: uint64_t = 2 as libc::c_int as uint64_t;
    while i_0 < MAX_CANDIDATE {
        let mut aux: *mut uint64_t =
            malloc(::std::mem::size_of::<uint64_t>() as libc::c_ulong) as *mut uint64_t;
        if !aux.is_null() {
        } else {
            __assert_fail(
                b"aux != NULL\x00" as *const u8 as *const libc::c_char,
                b"tests/primes_multiple_pools.c\x00" as *const u8 as *const libc::c_char,
                33 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00"))
                    .as_ptr(),
            );
        }
        *aux = i_0;
        thread_pool_add_work(
            *pools
                .as_mut_ptr()
                .offset(i_0.wrapping_rem(NUM_THREADPOOLS) as isize),
            Some(check_prime as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
            aux as *mut libc::c_void,
        );
        i_0 = i_0.wrapping_add(1)
    }
    let mut i_1: size_t = 0 as libc::c_int as size_t;
    while i_1 < NUM_THREADPOOLS {
        thread_pool_finish(*pools.as_mut_ptr().offset(i_1 as isize));
        i_1 = i_1.wrapping_add(1)
    }
    return 0;
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
