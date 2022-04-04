// #![allow(
//     dead_code,
//     mutable_transmutes,
//     non_camel_case_types,
//     non_snake_case,
//     non_upper_case_globals,
//     unused_assignments,
//     unused_mut
// )]
// #![register_tool(c2rust)]
// #![feature(extern_types, label_break_value, register_tool)]
// extern "C" {
//     pub type thread_pool;
//     #[no_mangle]
//     fn __assert_fail(
//         __assertion: *const libc::c_char,
//         __file: *const libc::c_char,
//         __line: libc::c_uint,
//         __function: *const libc::c_char,
//     ) -> !;
//     #[no_mangle]
//     fn pthread_cond_wait(__cond: *mut pthread_cond_t, __mutex: *mut pthread_mutex_t)
//         -> libc::c_int;
//     #[no_mangle]
//     fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
//     #[no_mangle]
//     fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
//     #[no_mangle]
//     fn pthread_cond_init(
//         __cond: *mut pthread_cond_t,
//         __cond_attr: *const pthread_condattr_t,
//     ) -> libc::c_int;
//     #[no_mangle]
//     fn pthread_mutex_init(
//         __mutex: *mut pthread_mutex_t,
//         __mutexattr: *const pthread_mutexattr_t,
//     ) -> libc::c_int;
//     #[no_mangle]
//     fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
//     #[no_mangle]
//     fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
//     #[no_mangle]
//     fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
//     #[no_mangle]
//     fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
//     #[no_mangle]
//     fn free(_: *mut libc::c_void);
//     #[no_mangle]
//     fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
//     #[no_mangle]
//     fn thread_pool_init(num_worker_threads: size_t) -> *mut thread_pool_t;
//     #[no_mangle]
//     fn thread_pool_add_work(
//         pool_0: *mut thread_pool_t,
//         function: work_function_t,
//         aux: *mut libc::c_void,
//     );
//     #[no_mangle]
//     fn thread_pool_finish(pool_0: *mut thread_pool_t);
// }
// pub type size_t = libc::c_ulong;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct __pthread_internal_list {
//     pub __prev: *mut __pthread_internal_list,
//     pub __next: *mut __pthread_internal_list,
// }
// pub type __pthread_list_t = __pthread_internal_list;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct __pthread_mutex_s {
//     pub __lock: libc::c_int,
//     pub __count: libc::c_uint,
//     pub __owner: libc::c_int,
//     pub __nusers: libc::c_uint,
//     pub __kind: libc::c_int,
//     pub __spins: libc::c_short,
//     pub __elision: libc::c_short,
//     pub __list: __pthread_list_t,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct __pthread_cond_s {
//     pub c2rust_unnamed: C2RustUnnamed_1,
//     pub c2rust_unnamed_0: C2RustUnnamed,
//     pub __g_refs: [libc::c_uint; 2],
//     pub __g_size: [libc::c_uint; 2],
//     pub __g1_orig_size: libc::c_uint,
//     pub __wrefs: libc::c_uint,
//     pub __g_signals: [libc::c_uint; 2],
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub union C2RustUnnamed {
//     pub __g1_start: libc::c_ulonglong,
//     pub __g1_start32: C2RustUnnamed_0,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct C2RustUnnamed_0 {
//     pub __low: libc::c_uint,
//     pub __high: libc::c_uint,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub union C2RustUnnamed_1 {
//     pub __wseq: libc::c_ulonglong,
//     pub __wseq32: C2RustUnnamed_2,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct C2RustUnnamed_2 {
//     pub __low: libc::c_uint,
//     pub __high: libc::c_uint,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub union pthread_mutexattr_t {
//     pub __size: [libc::c_char; 4],
//     pub __align: libc::c_int,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub union pthread_condattr_t {
//     pub __size: [libc::c_char; 4],
//     pub __align: libc::c_int,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub union pthread_mutex_t {
//     pub __data: __pthread_mutex_s,
//     pub __size: [libc::c_char; 40],
//     pub __align: libc::c_long,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub union pthread_cond_t {
//     pub __data: __pthread_cond_s,
//     pub __size: [libc::c_char; 48],
//     pub __align: libc::c_longlong,
// }
// pub type thread_pool_t = thread_pool;
// pub type work_function_t = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
// #[no_mangle]
// pub static mut NUM_THREADS: size_t = 16 as libc::c_int as size_t;
// #[no_mangle]
// pub static mut MAX_DEPTH: size_t = 5 as libc::c_int as size_t;
// #[no_mangle]
// pub static mut BRANCH_FACTOR: size_t = 5 as libc::c_int as size_t;
// #[no_mangle]
// pub static mut pool: *mut thread_pool_t = 0 as *const thread_pool_t as *mut thread_pool_t;
// #[no_mangle]
// pub static mut remaining_work: size_t = 0;
// #[no_mangle]
// pub static mut remaining_work_mutex: pthread_mutex_t = pthread_mutex_t {
//     __data: __pthread_mutex_s {
//         __lock: 0,
//         __count: 0,
//         __owner: 0,
//         __nusers: 0,
//         __kind: 0,
//         __spins: 0,
//         __elision: 0,
//         __list: __pthread_list_t {
//             __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
//             __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
//         },
//     },
// };
// #[no_mangle]
// pub static mut done_cond: pthread_cond_t = pthread_cond_t {
//     __data: __pthread_cond_s {
//         c2rust_unnamed: C2RustUnnamed_1 { __wseq: 0 },
//         c2rust_unnamed_0: C2RustUnnamed { __g1_start: 0 },
//         __g_refs: [0; 2],
//         __g_size: [0; 2],
//         __g1_orig_size: 0,
//         __wrefs: 0,
//         __g_signals: [0; 2],
//     },
// };
// #[no_mangle]
// pub unsafe extern "C" fn add_work(mut depth: size_t) {
//     let mut aux: *mut size_t =
//         malloc(::std::mem::size_of::<size_t>() as libc::c_ulong) as *mut size_t;
//     if !aux.is_null() {
//     } else {
//         __assert_fail(
//             b"aux != NULL\x00" as *const u8 as *const libc::c_char,
//             b"tests/recursive_add_work.c\x00" as *const u8 as *const libc::c_char,
//             21 as libc::c_int as libc::c_uint,
//             (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
//                 b"void add_work(size_t)\x00",
//             ))
//             .as_ptr(),
//         );
//     }
//     *aux = depth;
//     thread_pool_add_work(
//         pool,
//         Some(recursive_work as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
//         aux as *mut libc::c_void,
//     );
//     let mut result: libc::c_int = pthread_mutex_lock(&mut remaining_work_mutex);
//     if result == 0 as libc::c_int {
//     } else {
//         __assert_fail(
//             b"result == 0\x00" as *const u8 as *const libc::c_char,
//             b"tests/recursive_add_work.c\x00" as *const u8 as *const libc::c_char,
//             25 as libc::c_int as libc::c_uint,
//             (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
//                 b"void add_work(size_t)\x00",
//             ))
//             .as_ptr(),
//         );
//     }
//     remaining_work = remaining_work.wrapping_add(1);
//     result = pthread_mutex_unlock(&mut remaining_work_mutex);
//     if result == 0 as libc::c_int {
//     } else {
//         __assert_fail(
//             b"result == 0\x00" as *const u8 as *const libc::c_char,
//             b"tests/recursive_add_work.c\x00" as *const u8 as *const libc::c_char,
//             28 as libc::c_int as libc::c_uint,
//             (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
//                 b"void add_work(size_t)\x00",
//             ))
//             .as_ptr(),
//         );
//     };
// }
// #[no_mangle]
// pub unsafe extern "C" fn recursive_work(mut _depth: *mut libc::c_void) {
//     let mut depth: size_t = *(_depth as *mut size_t);
//     free(_depth);
//     if depth == MAX_DEPTH {
//         printf(b"Recursion depth hit!\n\x00" as *const u8 as *const libc::c_char);
//     } else {
//         let mut i: size_t = 0 as libc::c_int as size_t;
//         while i < BRANCH_FACTOR {
//             add_work(depth.wrapping_add(1 as libc::c_int as libc::c_ulong));
//             i = i.wrapping_add(1)
//         }
//     }
//     let mut result: libc::c_int = pthread_mutex_lock(&mut remaining_work_mutex);
//     if result == 0 as libc::c_int {
//     } else {
//         __assert_fail(
//             b"result == 0\x00" as *const u8 as *const libc::c_char,
//             b"tests/recursive_add_work.c\x00" as *const u8 as *const libc::c_char,
//             45 as libc::c_int as libc::c_uint,
//             (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
//                 b"void recursive_work(void *)\x00",
//             ))
//             .as_ptr(),
//         );
//     }
//     remaining_work = remaining_work.wrapping_sub(1);
//     if remaining_work == 0 as libc::c_int as libc::c_ulong {
//         pthread_cond_signal(&mut done_cond);
//     }
//     result = pthread_mutex_unlock(&mut remaining_work_mutex);
//     if result == 0 as libc::c_int {
//     } else {
//         __assert_fail(
//             b"result == 0\x00" as *const u8 as *const libc::c_char,
//             b"tests/recursive_add_work.c\x00" as *const u8 as *const libc::c_char,
//             50 as libc::c_int as libc::c_uint,
//             (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
//                 b"void recursive_work(void *)\x00",
//             ))
//             .as_ptr(),
//         );
//     };
// }
// unsafe fn main_0() -> libc::c_int {
//     pool = thread_pool_init(NUM_THREADS);
//     remaining_work = 0 as libc::c_int as size_t;
//     let mut result: libc::c_int =
//         pthread_mutex_init(&mut remaining_work_mutex, 0 as *const pthread_mutexattr_t);
//     if result == 0 as libc::c_int {
//     } else {
//         __assert_fail(
//             b"result == 0\x00" as *const u8 as *const libc::c_char,
//             b"tests/recursive_add_work.c\x00" as *const u8 as *const libc::c_char,
//             57 as libc::c_int as libc::c_uint,
//             (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00")).as_ptr(),
//         );
//     }
//     result = pthread_cond_init(&mut done_cond, 0 as *const pthread_condattr_t);
//     if result == 0 as libc::c_int {
//     } else {
//         __assert_fail(
//             b"result == 0\x00" as *const u8 as *const libc::c_char,
//             b"tests/recursive_add_work.c\x00" as *const u8 as *const libc::c_char,
//             59 as libc::c_int as libc::c_uint,
//             (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00")).as_ptr(),
//         );
//     }
//     add_work(0 as libc::c_int as size_t);
//     result = pthread_mutex_lock(&mut remaining_work_mutex);
//     if result == 0 as libc::c_int {
//     } else {
//         __assert_fail(
//             b"result == 0\x00" as *const u8 as *const libc::c_char,
//             b"tests/recursive_add_work.c\x00" as *const u8 as *const libc::c_char,
//             63 as libc::c_int as libc::c_uint,
//             (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00")).as_ptr(),
//         );
//     }
//     while remaining_work > 0 as libc::c_int as libc::c_ulong {
//         result = pthread_cond_wait(&mut done_cond, &mut remaining_work_mutex);
//         if result == 0 as libc::c_int {
//         } else {
//             __assert_fail(
//                 b"result == 0\x00" as *const u8 as *const libc::c_char,
//                 b"tests/recursive_add_work.c\x00" as *const u8 as *const libc::c_char,
//                 66 as libc::c_int as libc::c_uint,
//                 (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00"))
//                     .as_ptr(),
//             );
//         }
//     }
//     result = pthread_mutex_unlock(&mut remaining_work_mutex);
//     if result == 0 as libc::c_int {
//     } else {
//         __assert_fail(
//             b"result == 0\x00" as *const u8 as *const libc::c_char,
//             b"tests/recursive_add_work.c\x00" as *const u8 as *const libc::c_char,
//             69 as libc::c_int as libc::c_uint,
//             (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00")).as_ptr(),
//         );
//     }
//     thread_pool_finish(pool);
//     result = pthread_mutex_destroy(&mut remaining_work_mutex);
//     if result == 0 as libc::c_int {
//     } else {
//         __assert_fail(
//             b"result == 0\x00" as *const u8 as *const libc::c_char,
//             b"tests/recursive_add_work.c\x00" as *const u8 as *const libc::c_char,
//             73 as libc::c_int as libc::c_uint,
//             (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00")).as_ptr(),
//         );
//     }
//     result = pthread_cond_destroy(&mut done_cond);
//     if result == 0 as libc::c_int {
//     } else {
//         __assert_fail(
//             b"result == 0\x00" as *const u8 as *const libc::c_char,
//             b"tests/recursive_add_work.c\x00" as *const u8 as *const libc::c_char,
//             75 as libc::c_int as libc::c_uint,
//             (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00")).as_ptr(),
//         );
//     }
//     return 0;
// }

// pub fn main() {
//     unsafe { ::std::process::exit(main_0() as i32) }
// }
