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

    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;

    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;

    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;

    fn free(_: *mut libc::c_void);

    fn nanosleep(__requested_time: *const timespec, __remaining: *mut timespec) -> libc::c_int;

    fn thread_pool_init(num_worker_threads: size_t) -> *mut thread_pool_t;

    fn thread_pool_add_work(
        pool: *mut thread_pool_t,
        function: work_function_t,
        aux: *mut libc::c_void,
    );

    fn thread_pool_finish(pool: *mut thread_pool_t);
}
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type thread_pool_t = thread_pool;
pub type work_function_t = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[no_mangle]
pub static mut MAX_CANDIDATE: uint64_t = 100000 as libc::c_int as uint64_t;
#[no_mangle]
pub static mut NUM_THREADS: size_t = 16 as libc::c_int as size_t;
#[no_mangle]
pub static mut WORK_DELAY: timespec = {
    let mut init = timespec {
        tv_sec: 0 as libc::c_int as __time_t,
        tv_nsec: 100000 as libc::c_int as __syscall_slong_t,
    };
    init
};
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
    let mut pool: *mut thread_pool_t = thread_pool_init(NUM_THREADS);
    let mut i: uint64_t = 2 as libc::c_int as uint64_t;
    while i < MAX_CANDIDATE {
        let mut aux: *mut uint64_t =
            malloc(::std::mem::size_of::<uint64_t>() as libc::c_ulong) as *mut uint64_t;
        if !aux.is_null() {
        } else {
            __assert_fail(
                b"aux != NULL\x00" as *const u8 as *const libc::c_char,
                b"tests/primes_periodic_work.c\x00" as *const u8 as *const libc::c_char,
                30 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00"))
                    .as_ptr(),
            );
        }
        *aux = i;
        thread_pool_add_work(
            pool,
            Some(check_prime as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
            aux as *mut libc::c_void,
        );
        let mut result: libc::c_int = nanosleep(&WORK_DELAY, 0 as *mut timespec);
        if result == 0 as libc::c_int {
        } else {
            __assert_fail(
                b"result == 0\x00" as *const u8 as *const libc::c_char,
                b"tests/primes_periodic_work.c\x00" as *const u8 as *const libc::c_char,
                34 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00"))
                    .as_ptr(),
            );
        }
        i = i.wrapping_add(1)
    }
    thread_pool_finish(pool);
    return 0;
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
