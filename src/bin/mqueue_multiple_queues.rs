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
    fn nanosleep(__requested_time: *const timespec, __remaining: *mut timespec) -> libc::c_int;
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
    fn queue_init() -> *mut queue_t;
    #[no_mangle]
    fn queue_enqueue(queue: *mut queue_t, value: *mut libc::c_void);
    #[no_mangle]
    fn queue_dequeue(queue: *mut queue_t) -> *mut libc::c_void;
    #[no_mangle]
    fn queue_free(queue: *mut queue_t);
}
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type queue_t = queue;
#[no_mangle]
pub static mut QUEUES: size_t = 10 as libc::c_int as size_t;
#[no_mangle]
pub static mut QUEUE_VALUES: size_t = 10 as libc::c_int as size_t;
#[no_mangle]
pub static mut REPETITIONS: size_t = 100 as libc::c_int as size_t;
#[no_mangle]
pub static mut ENQUEUE_DELAY: timespec = {
    let mut init = timespec {
        tv_sec: 0 as libc::c_int as __time_t,
        tv_nsec: 1000000 as libc::c_int as __syscall_slong_t,
    };
    init
};
#[no_mangle]
pub static mut queues: [*mut queue_t; 10] = [0 as *const queue_t as *mut queue_t; 10];
#[no_mangle]
pub unsafe extern "C" fn enqueuer(mut _queue_index: *mut libc::c_void) -> *mut libc::c_void {
    let mut queue_index: size_t = _queue_index as size_t;
    let mut queue: *mut queue_t = queues[queue_index as usize];
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < QUEUE_VALUES {
        let mut value: size_t = queue_index.wrapping_add(i.wrapping_mul(QUEUES));
        queue_enqueue(queue, value as *mut libc::c_void);
        let mut result: libc::c_int = nanosleep(&ENQUEUE_DELAY, 0 as *mut timespec);
        if result == 0 as libc::c_int {
        } else {
            __assert_fail(
                b"result == 0\x00" as *const u8 as *const libc::c_char,
                b"tests/mqueue_multiple_queues.c\x00" as *const u8 as *const libc::c_char,
                21 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                    b"void *enqueuer(void *)\x00",
                ))
                .as_ptr(),
            );
        }
        i = i.wrapping_add(1)
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn dequeuer(mut _queue_index: *mut libc::c_void) -> *mut libc::c_void {
    let mut queue_index: size_t = _queue_index as size_t;
    let mut queue: *mut queue_t = queues[queue_index as usize];
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < QUEUE_VALUES {
        let mut value: size_t = queue_dequeue(queue) as size_t;
        if value == queue_index.wrapping_add(i.wrapping_mul(QUEUES)) {
        } else {
            __assert_fail(
                b"value == queue_index + i * QUEUES\x00" as *const u8 as *const libc::c_char,
                b"tests/mqueue_multiple_queues.c\x00" as *const u8 as *const libc::c_char,
                31 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                    b"void *dequeuer(void *)\x00",
                ))
                .as_ptr(),
            );
        }
        i = i.wrapping_add(1)
    }
    return 0 as *mut libc::c_void;
}
unsafe fn main_0() -> libc::c_int {
    let mut repetition: size_t = 0 as libc::c_int as size_t;
    while repetition < REPETITIONS {
        let vla = QUEUES as usize;
        let mut enqueue_threads: Vec<pthread_t> = ::std::vec::from_elem(0, vla);
        let vla_0 = QUEUES as usize;
        let mut dequeue_threads: Vec<pthread_t> = ::std::vec::from_elem(0, vla_0);
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < QUEUES {
            queues[i as usize] = queue_init();
            let mut result: libc::c_int = pthread_create(
                &mut *dequeue_threads.as_mut_ptr().offset(i as isize),
                0 as *const pthread_attr_t,
                Some(dequeuer as unsafe extern "C" fn(_: *mut libc::c_void) -> *mut libc::c_void),
                i as *mut libc::c_void,
            );
            if result == 0 as libc::c_int {
            } else {
                __assert_fail(
                    b"result == 0\x00" as *const u8 as *const libc::c_char,
                    b"tests/mqueue_multiple_queues.c\x00" as *const u8 as *const libc::c_char,
                    43 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00"))
                        .as_ptr(),
                );
            }
            result = pthread_create(
                &mut *enqueue_threads.as_mut_ptr().offset(i as isize),
                0 as *const pthread_attr_t,
                Some(enqueuer as unsafe extern "C" fn(_: *mut libc::c_void) -> *mut libc::c_void),
                i as *mut libc::c_void,
            );
            if result == 0 as libc::c_int {
            } else {
                __assert_fail(
                    b"result == 0\x00" as *const u8 as *const libc::c_char,
                    b"tests/mqueue_multiple_queues.c\x00" as *const u8 as *const libc::c_char,
                    45 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00"))
                        .as_ptr(),
                );
            }
            i = i.wrapping_add(1)
        }
        let mut i_0: size_t = 0 as libc::c_int as size_t;
        while i_0 < QUEUES {
            let mut result_0: libc::c_int = pthread_join(
                *enqueue_threads.as_mut_ptr().offset(i_0 as isize),
                0 as *mut *mut libc::c_void,
            );
            if result_0 == 0 as libc::c_int {
            } else {
                __assert_fail(
                    b"result == 0\x00" as *const u8 as *const libc::c_char,
                    b"tests/mqueue_multiple_queues.c\x00" as *const u8 as *const libc::c_char,
                    49 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00"))
                        .as_ptr(),
                );
            }
            result_0 = pthread_join(
                *dequeue_threads.as_mut_ptr().offset(i_0 as isize),
                0 as *mut *mut libc::c_void,
            );
            if result_0 == 0 as libc::c_int {
            } else {
                __assert_fail(
                    b"result == 0\x00" as *const u8 as *const libc::c_char,
                    b"tests/mqueue_multiple_queues.c\x00" as *const u8 as *const libc::c_char,
                    51 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00"))
                        .as_ptr(),
                );
            }
            queue_free(queues[i_0 as usize]);
            i_0 = i_0.wrapping_add(1)
        }
        repetition = repetition.wrapping_add(1)
    }
    return 0;
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
