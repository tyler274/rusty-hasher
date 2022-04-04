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
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;

    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;

    fn free(_: *mut libc::c_void);

    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;

    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> *mut libc::c_void>,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;

    fn pthread_join(__th: pthread_t, __thread_return: *mut *mut libc::c_void) -> libc::c_int;

    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;

    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;

    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;

    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;

    fn queue_init() -> *mut queue_t;

    fn queue_enqueue(queue_0: *mut queue_t, value: *mut libc::c_void);

    fn queue_dequeue(queue_0: *mut queue_t) -> *mut libc::c_void;

    fn queue_free(queue_0: *mut queue_t);
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type queue_t = queue;
#[no_mangle]
pub static mut MAX_QUEUE_SIZE: size_t = 100000 as libc::c_int as size_t;
#[no_mangle]
pub static mut NUM_PUSH_THREADS: size_t = 8 as libc::c_int as size_t;
#[no_mangle]
pub static mut NUM_POP_THREADS: size_t = 8 as libc::c_int as size_t;
#[no_mangle]
pub static mut queue: *mut queue_t = 0 as *const queue_t as *mut queue_t;
#[no_mangle]
pub static mut num_seen: size_t = 0;
#[no_mangle]
pub static mut num_seen_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0,
        __count: 0,
        __owner: 0,
        __nusers: 0,
        __kind: 0,
        __spins: 0,
        __elision: 0,
        __list: __pthread_list_t {
            __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
            __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
        },
    },
};
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
                b"tests/mqueue_push_pop.c\x00" as *const u8 as *const libc::c_char,
                22 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                    b"void *enqueue_worker(void *)\x00",
                ))
                .as_ptr(),
            );
        }
        *value = i;
        queue_enqueue(queue, value as *mut libc::c_void);
        i = (i as libc::c_ulong).wrapping_add(NUM_PUSH_THREADS) as size_t as size_t
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn dequeue_worker(mut _arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < MAX_QUEUE_SIZE {
        pthread_mutex_lock(&mut num_seen_mutex);
        if num_seen == MAX_QUEUE_SIZE {
            pthread_mutex_unlock(&mut num_seen_mutex);
            break;
        } else {
            num_seen = num_seen.wrapping_add(1);
            pthread_mutex_unlock(&mut num_seen_mutex);
            let mut value: *mut size_t = queue_dequeue(queue) as *mut size_t;
            printf(b"%zu\n\x00" as *const u8 as *const libc::c_char, *value);
            free(value as *mut libc::c_void);
            i = i.wrapping_add(1)
        }
    }
    return 0 as *mut libc::c_void;
}
unsafe fn main_0() -> libc::c_int {
    queue = queue_init();
    num_seen = 0 as libc::c_int as size_t;
    let mut result: libc::c_int =
        pthread_mutex_init(&mut num_seen_mutex, 0 as *const pthread_mutexattr_t);
    if result == 0 as libc::c_int {
    } else {
        __assert_fail(
            b"result == 0\x00" as *const u8 as *const libc::c_char,
            b"tests/mqueue_push_pop.c\x00" as *const u8 as *const libc::c_char,
            52 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00")).as_ptr(),
        );
    }
    let vla = NUM_PUSH_THREADS as usize;
    let mut enqueue_threads: Vec<pthread_t> = ::std::vec::from_elem(0, vla);
    let vla_0 = NUM_POP_THREADS as usize;
    let mut dequeue_threads: Vec<pthread_t> = ::std::vec::from_elem(0, vla_0);
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < NUM_PUSH_THREADS {
        let mut thread_id: *mut size_t =
            malloc(::std::mem::size_of::<size_t>() as libc::c_ulong) as *mut size_t;
        if !thread_id.is_null() {
        } else {
            __assert_fail(
                b"thread_id != NULL\x00" as *const u8 as *const libc::c_char,
                b"tests/mqueue_push_pop.c\x00" as *const u8 as *const libc::c_char,
                59 as libc::c_int as libc::c_uint,
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
                b"tests/mqueue_push_pop.c\x00" as *const u8 as *const libc::c_char,
                62 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00"))
                    .as_ptr(),
            );
        }
        i = i.wrapping_add(1)
    }
    let mut i_0: size_t = 0 as libc::c_int as size_t;
    while i_0 < NUM_POP_THREADS {
        result = pthread_create(
            &mut *dequeue_threads.as_mut_ptr().offset(i_0 as isize),
            0 as *const pthread_attr_t,
            Some(dequeue_worker as unsafe extern "C" fn(_: *mut libc::c_void) -> *mut libc::c_void),
            0 as *mut libc::c_void,
        );
        if result == 0 as libc::c_int {
        } else {
            __assert_fail(
                b"result == 0\x00" as *const u8 as *const libc::c_char,
                b"tests/mqueue_push_pop.c\x00" as *const u8 as *const libc::c_char,
                66 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00"))
                    .as_ptr(),
            );
        }
        i_0 = i_0.wrapping_add(1)
    }
    let mut i_1: size_t = 0 as libc::c_int as size_t;
    while i_1 < NUM_PUSH_THREADS {
        result = pthread_join(
            *enqueue_threads.as_mut_ptr().offset(i_1 as isize),
            0 as *mut *mut libc::c_void,
        );
        if result == 0 as libc::c_int {
        } else {
            __assert_fail(
                b"result == 0\x00" as *const u8 as *const libc::c_char,
                b"tests/mqueue_push_pop.c\x00" as *const u8 as *const libc::c_char,
                71 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00"))
                    .as_ptr(),
            );
        }
        i_1 = i_1.wrapping_add(1)
    }
    let mut i_2: size_t = 0 as libc::c_int as size_t;
    while i_2 < NUM_POP_THREADS {
        result = pthread_join(
            *dequeue_threads.as_mut_ptr().offset(i_2 as isize),
            0 as *mut *mut libc::c_void,
        );
        if result == 0 as libc::c_int {
        } else {
            __assert_fail(
                b"result == 0\x00" as *const u8 as *const libc::c_char,
                b"tests/mqueue_push_pop.c\x00" as *const u8 as *const libc::c_char,
                75 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00"))
                    .as_ptr(),
            );
        }
        i_2 = i_2.wrapping_add(1)
    }
    result = pthread_mutex_destroy(&mut num_seen_mutex);
    if result == 0 as libc::c_int {
    } else {
        __assert_fail(
            b"result == 0\x00" as *const u8 as *const libc::c_char,
            b"tests/mqueue_push_pop.c\x00" as *const u8 as *const libc::c_char,
            79 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00")).as_ptr(),
        );
    }
    queue_free(queue);
    return 0;
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
