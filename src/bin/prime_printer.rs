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
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type thread_pool;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strtoul(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
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
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type thread_pool_t = thread_pool;
pub type work_function_t = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[no_mangle]
pub static mut MAX_CANDIDATE: uint64_t = 100000 as libc::c_int as uint64_t;
#[no_mangle]
pub unsafe extern "C" fn usage(mut argv: *mut *mut libc::c_char) {
    fprintf(
        stderr,
        b"Usage: %s <# of threads>\n\x00" as *const u8 as *const libc::c_char,
        *argv.offset(0 as libc::c_int as isize),
    );
    exit(1 as libc::c_int);
}
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
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    if argc != 2 as libc::c_int {
        usage(argv);
    }
    let mut num_threads: size_t = strtoul(
        *argv.offset(1 as libc::c_int as isize),
        0 as *mut *mut libc::c_char,
        0 as libc::c_int,
    );
    if num_threads == 0 as libc::c_int as libc::c_ulong {
        usage(argv);
    }
    let mut pool: *mut thread_pool_t = thread_pool_init(num_threads);
    let mut i: uint64_t = 2 as libc::c_int as uint64_t;
    while i < MAX_CANDIDATE {
        let mut aux: *mut uint64_t =
            malloc(::std::mem::size_of::<uint64_t>() as libc::c_ulong) as *mut uint64_t;
        if !aux.is_null() {
        } else {
            __assert_fail(
                b"aux != NULL\x00" as *const u8 as *const libc::c_char,
                b"tests/prime_printer.c\x00" as *const u8 as *const libc::c_char,
                41 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                    b"int main(int, char **)\x00",
                ))
                .as_ptr(),
            );
        }
        *aux = i;
        thread_pool_add_work(
            pool,
            Some(check_prime as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
            aux as *mut libc::c_void,
        );
        i = i.wrapping_add(1)
    }
    thread_pool_finish(pool);
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
