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
//     pub type queue;
//     #[no_mangle]
//     fn __assert_fail(
//         __assertion: *const libc::c_char,
//         __file: *const libc::c_char,
//         __line: libc::c_uint,
//         __function: *const libc::c_char,
//     ) -> !;
//     #[no_mangle]
//     fn rand() -> libc::c_int;
//     #[no_mangle]
//     fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
//     #[no_mangle]
//     fn free(_: *mut libc::c_void);
//     #[no_mangle]
//     fn queue_init() -> *mut queue_t;
//     #[no_mangle]
//     fn queue_enqueue(queue: *mut queue_t, value: *mut libc::c_void);
//     #[no_mangle]
//     fn queue_dequeue(queue: *mut queue_t) -> *mut libc::c_void;
//     #[no_mangle]
//     fn queue_free(queue: *mut queue_t);
// }
// pub type size_t = libc::c_ulong;
// pub type queue_t = queue;
// #[no_mangle]
// pub static mut MAX_QUEUE_SIZE: size_t = 100000 as libc::c_int as size_t;
// unsafe fn main_0() -> libc::c_int {
//     let mut queue: *mut queue_t = queue_init();
//     let mut push_idx: size_t = 0 as libc::c_int as size_t;
//     let mut pop_idx: size_t = 0 as libc::c_int as size_t;
//     while push_idx < MAX_QUEUE_SIZE {
//         // Randomly enqueue a burst of elements.
//         let mut push_burst_end: size_t =
//             push_idx.wrapping_add((rand() % 1000 as libc::c_int) as libc::c_ulong);
//         if push_burst_end > MAX_QUEUE_SIZE {
//             push_burst_end = MAX_QUEUE_SIZE
//         }
//         while push_idx < push_burst_end {
//             let mut value: *mut size_t =
//                 malloc(::std::mem::size_of::<size_t>() as libc::c_ulong) as *mut size_t;
//             if !value.is_null() {
//             } else {
//                 __assert_fail(
//                     b"value != NULL\x00" as *const u8 as *const libc::c_char,
//                     b"tests/squeue_push_pop.c\x00" as *const u8 as *const libc::c_char,
//                     22 as libc::c_int as libc::c_uint,
//                     (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00"))
//                         .as_ptr(),
//                 );
//             }
//             *value = push_idx;
//             queue_enqueue(queue, value as *mut libc::c_void);
//             push_idx = push_idx.wrapping_add(1)
//         }
//         // Randomly dequeue a burst of elements, making sure to not
//         // dequeue more than we're previously enqueued.
//         let mut pop_burst_end: size_t =
//             pop_idx.wrapping_add((rand() % 1000 as libc::c_int) as libc::c_ulong);
//         if pop_burst_end >= push_burst_end {
//             pop_burst_end = push_burst_end
//         }
//         while pop_idx < pop_burst_end {
//             let mut value_0: *mut size_t = queue_dequeue(queue) as *mut size_t;
//             if *value_0 == pop_idx {
//             } else {
//                 __assert_fail(
//                     b"*value == pop_idx\x00" as *const u8 as *const libc::c_char,
//                     b"tests/squeue_push_pop.c\x00" as *const u8 as *const libc::c_char,
//                     36 as libc::c_int as libc::c_uint,
//                     (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00"))
//                         .as_ptr(),
//                 );
//             }
//             free(value_0 as *mut libc::c_void);
//             pop_idx = pop_idx.wrapping_add(1)
//         }
//     }
//     // If remaining elements are left on the queue, verify order.
//     while pop_idx < MAX_QUEUE_SIZE {
//         let mut value_1: *mut size_t = queue_dequeue(queue) as *mut size_t;
//         if *value_1 == pop_idx {
//         } else {
//             __assert_fail(
//                 b"*value == pop_idx\x00" as *const u8 as *const libc::c_char,
//                 b"tests/squeue_push_pop.c\x00" as *const u8 as *const libc::c_char,
//                 45 as libc::c_int as libc::c_uint,
//                 (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00"))
//                     .as_ptr(),
//             );
//         }
//         free(value_1 as *mut libc::c_void);
//         pop_idx = pop_idx.wrapping_add(1)
//     }
//     queue_free(queue);
//     return 0;
// }

// pub fn main() {
//     unsafe { ::std::process::exit(main_0() as i32) }
// }
