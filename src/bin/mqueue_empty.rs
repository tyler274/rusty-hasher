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
#![feature(
    extern_types,
    label_break_value,
    register_tool
)]
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
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> *mut libc::c_void>,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    #[no_mangle]
    fn pthread_join(__th: pthread_t, __thread_return: *mut *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    fn queue_init() -> *mut queue_t;
    #[no_mangle]
    fn queue_enqueue(queue_0: *mut queue_t, value: *mut libc::c_void);
    #[no_mangle]
    fn queue_dequeue(queue_0: *mut queue_t) -> *mut libc::c_void;
    #[no_mangle]
    fn queue_free(queue_0: *mut queue_t);
}
pub type size_t = libc::c_ulong;
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type queue_t = queue;
#[no_mangle]
pub static mut MAX_QUEUE_SIZE: size_t = 100000 as libc::c_int as size_t;
#[no_mangle]
pub static mut NUM_THREADS: size_t = 16 as libc::c_int as size_t;
#[no_mangle]
pub static mut queue: *mut queue_t = 0 as *const queue_t as *mut queue_t;
#[no_mangle]
pub unsafe extern "C" fn enqueue_worker(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut thread_id: size_t = *(arg as *mut size_t);
    free(arg);
    let mut i: size_t = thread_id;
    while i < MAX_QUEUE_SIZE {
        let mut value: *mut size_t =
            malloc(::std::mem::size_of::<size_t>() as libc::c_ulong) as *mut size_t;
        if !value.is_null() {
        } else {
            __assert_fail(
                b"value != NULL\x00" as *const u8 as *const libc::c_char,
                b"tests/mqueue_empty.c\x00" as *const u8 as *const libc::c_char,
                21 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                    b"void *enqueue_worker(void *)\x00",
                ))
                .as_ptr(),
            );
        }
        *value = i;
        queue_enqueue(queue, value as *mut libc::c_void);
        i = (i as libc::c_ulong).wrapping_add(NUM_THREADS) as size_t as size_t
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn dequeue_worker(mut _arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut thread_indices: [size_t; 16] = [
        0 as libc::c_int as size_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < MAX_QUEUE_SIZE {
        let mut value: *mut size_t = queue_dequeue(queue) as *mut size_t;
        let mut thread_id: size_t = (*value).wrapping_rem(NUM_THREADS);
        let fresh0 = thread_indices[thread_id as usize];
        thread_indices[thread_id as usize] = thread_indices[thread_id as usize].wrapping_add(1);
        let mut thread_index: size_t = fresh0;
        if *value
            == thread_index
                .wrapping_mul(NUM_THREADS)
                .wrapping_add(thread_id)
        {
        } else {
            __assert_fail(
                b"*value == thread_index * NUM_THREADS + thread_id\x00" as *const u8
                    as *const libc::c_char,
                b"tests/mqueue_empty.c\x00" as *const u8 as *const libc::c_char,
                38 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                    b"void *dequeue_worker(void *)\x00",
                ))
                .as_ptr(),
            );
        }
        free(value as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    let mut i_0: size_t = 0 as libc::c_int as size_t;
    while i_0 < NUM_THREADS {
        if thread_indices[i_0 as usize] == MAX_QUEUE_SIZE.wrapping_div(NUM_THREADS) {
        } else {
            __assert_fail(
                b"thread_indices[i] == MAX_QUEUE_SIZE / NUM_THREADS\x00" as *const u8
                    as *const libc::c_char,
                b"tests/mqueue_empty.c\x00" as *const u8 as *const libc::c_char,
                44 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                    b"void *dequeue_worker(void *)\x00",
                ))
                .as_ptr(),
            );
        }
        i_0 = i_0.wrapping_add(1)
    }
    return 0 as *mut libc::c_void;
}
unsafe fn main_0() -> libc::c_int {
    queue = queue_init();
    let vla = NUM_THREADS as usize;
    let mut enqueue_threads: Vec<pthread_t> = ::std::vec::from_elem(0, vla);
    let mut dequeue_thread: pthread_t = 0;
    let mut result: libc::c_int = pthread_create(
        &mut dequeue_thread,
        0 as *const pthread_attr_t,
        Some(dequeue_worker as unsafe extern "C" fn(_: *mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    );
    if result == 0 as libc::c_int {
    } else {
        __assert_fail(
            b"result == 0\x00" as *const u8 as *const libc::c_char,
            b"tests/mqueue_empty.c\x00" as *const u8 as *const libc::c_char,
            57 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00")).as_ptr(),
        );
    }
    sleep(5 as libc::c_int as libc::c_uint);
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < NUM_THREADS {
        let mut thread_id: *mut size_t =
            malloc(::std::mem::size_of::<size_t>() as libc::c_ulong) as *mut size_t;
        if !thread_id.is_null() {
        } else {
            __assert_fail(
                b"thread_id != NULL\x00" as *const u8 as *const libc::c_char,
                b"tests/mqueue_empty.c\x00" as *const u8 as *const libc::c_char,
                62 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00"))
                    .as_ptr(),
            );
        }
        *thread_id = i;
        result = pthread_create(
            &mut *enqueue_threads.as_mut_ptr().offset(i as isize),
            0 as *const pthread_attr_t,
            Some(enqueue_worker as unsafe extern "C" fn(_: *mut libc::c_void) -> *mut libc::c_void),
            thread_id as *mut libc::c_void,
        );
        if result == 0 as libc::c_int {
        } else {
            __assert_fail(
                b"result == 0\x00" as *const u8 as *const libc::c_char,
                b"tests/mqueue_empty.c\x00" as *const u8 as *const libc::c_char,
                65 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00"))
                    .as_ptr(),
            );
        }
        i = i.wrapping_add(1)
    }
    let mut i_0: size_t = 0 as libc::c_int as size_t;
    while i_0 < NUM_THREADS {
        result = pthread_join(
            *enqueue_threads.as_mut_ptr().offset(i_0 as isize),
            0 as *mut *mut libc::c_void,
        );
        if result == 0 as libc::c_int {
        } else {
            __assert_fail(
                b"result == 0\x00" as *const u8 as *const libc::c_char,
                b"tests/mqueue_empty.c\x00" as *const u8 as *const libc::c_char,
                70 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00"))
                    .as_ptr(),
            );
        }
        i_0 = i_0.wrapping_add(1)
    }
    result = pthread_join(dequeue_thread, 0 as *mut *mut libc::c_void);
    if result == 0 as libc::c_int {
    } else {
        __assert_fail(
            b"result == 0\x00" as *const u8 as *const libc::c_char,
            b"tests/mqueue_empty.c\x00" as *const u8 as *const libc::c_char,
            73 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00")).as_ptr(),
        );
    }
    queue_free(queue);
    return 0;
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
