#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use rusty_hasher::thread_pool::{
    thread_pool_add_work, thread_pool_finish, thread_pool_init, thread_pool_t,
};

// pub type size_t = libc::c_ulong;
// pub type __off_t = libc::c_long;
// pub type __off64_t = libc::c_long;
// pub type __time_t = libc::c_long;
// pub type __clockid_t = libc::c_int;
// pub type __syscall_slong_t = libc::c_long;

pub type work_function_t = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[no_mangle]
pub static mut NS_PER_SEC: libc::time_t = 1e9f64 as libc::time_t;
#[no_mangle]
pub static mut NUM_SLEEPS: libc::size_t = 10 as libc::c_int as libc::size_t;
#[no_mangle]
pub unsafe extern "C" fn usage(mut argv: *mut *mut libc::c_char) {
    let mode = std::ffi::CString::new("w").unwrap();
    libc::fprintf(
        libc::fdopen(libc::STDERR_FILENO, mode.as_ptr()),
        b"Usage: %s <# of threads>\n\x00" as *const u8 as *const libc::c_char,
        *argv.offset(0 as libc::c_int as isize),
    );
    libc::exit(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn sleeper(mut aux: *mut libc::c_void) {
    libc::sleep(1 as libc::c_int as libc::c_uint);
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    if argc != 2 as libc::c_int {
        usage(argv);
    }
    let mut num_threads: libc::size_t = libc::strtoul(
        *argv.offset(1 as libc::c_int as isize),
        0 as *mut *mut libc::c_char,
        0 as libc::c_int,
    ) as libc::size_t;
    if num_threads == 0 {
        usage(argv);
    }
    let mut start: libc::timespec = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut result: libc::c_int = libc::clock_gettime(0 as libc::c_int, &mut start);
    assert!(result == 0);
    let mut pool: *mut thread_pool_t = thread_pool_init(num_threads);
    let mut i: libc::size_t = 0;
    while i < NUM_SLEEPS {
        thread_pool_add_work(
            pool,
            Some(sleeper as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
            0 as *mut libc::c_void,
        );
        i = i.wrapping_add(1)
    }
    thread_pool_finish(pool);
    let mut end: libc::timespec = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    result = libc::clock_gettime(0 as libc::c_int, &mut end);
    assert!(result == 0);
    let mut duration: libc::time_t =
        end.tv_sec - start.tv_sec + (end.tv_nsec - start.tv_nsec) / NS_PER_SEC;
    libc::printf(b"%lu\n\x00" as *const u8 as *const libc::c_char, duration);
    return 0;
}

pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
