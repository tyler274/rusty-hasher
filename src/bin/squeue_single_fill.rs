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
    pub type queue;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn queue_init() -> *mut queue_t;
    #[no_mangle]
    fn queue_enqueue(queue: *mut queue_t, value: *mut libc::c_void);
    #[no_mangle]
    fn queue_dequeue(queue: *mut queue_t) -> *mut libc::c_void;
    #[no_mangle]
    fn queue_free(queue: *mut queue_t);
}
pub type size_t = libc::c_ulong;
pub type queue_t = queue;
#[no_mangle]
pub static mut MAX_QUEUE_SIZE: size_t = 100000 as libc::c_int as size_t;
unsafe fn main_0() -> libc::c_int {
    let mut queue: *mut queue_t = queue_init();
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < MAX_QUEUE_SIZE {
        let mut value: *mut size_t =
            malloc(::std::mem::size_of::<size_t>() as libc::c_ulong) as *mut size_t;
        if !value.is_null() {
        } else {
            __assert_fail(
                b"value != NULL\x00" as *const u8 as *const libc::c_char,
                b"tests/squeue_single_fill.c\x00" as *const u8 as *const libc::c_char,
                13 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00"))
                    .as_ptr(),
            );
        }
        *value = i;
        queue_enqueue(queue, value as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    let mut i_0: size_t = 0 as libc::c_int as size_t;
    while i_0 < MAX_QUEUE_SIZE {
        let mut value_0: *mut size_t = queue_dequeue(queue) as *mut size_t;
        if *value_0 == i_0 {
        } else {
            __assert_fail(
                b"*value == i\x00" as *const u8 as *const libc::c_char,
                b"tests/squeue_single_fill.c\x00" as *const u8 as *const libc::c_char,
                20 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00"))
                    .as_ptr(),
            );
        }
        free(value_0 as *mut libc::c_void);
        i_0 = i_0.wrapping_add(1)
    }
    queue_free(queue);
    return 0;
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
