#![feature(scoped_threads)]
use scoped_threadpool_std::ThreadPool;

// #[no_mangle]
// pub static mut QUEUES: size_t = 10 as libc::c_int as size_t;
// #[no_mangle]
// pub static mut QUEUE_VALUES: size_t = 10 as libc::c_int as size_t;
// #[no_mangle]
// pub static mut REPETITIONS: size_t = 100 as libc::c_int as size_t;
// #[no_mangle]
// pub static mut ENQUEUE_DELAY: timespec = {
//     let mut init = timespec {
//         tv_sec: 0 as libc::c_int as __time_t,
//         tv_nsec: 1000000 as libc::c_int as __syscall_slong_t,
//     };
//     init
// };
// #[no_mangle]
// pub static mut queues: [*mut queue_t; 10] = [0 as *const queue_t as *mut queue_t; 10];
// #[no_mangle]
// pub unsafe extern "C" fn enqueuer(mut _queue_index: *mut libc::c_void) -> *mut libc::c_void {
//     let mut queue_index: size_t = _queue_index as size_t;
//     let mut queue: *mut queue_t = queues[queue_index as usize];
//     let mut i: size_t = 0 as libc::c_int as size_t;
//     while i < QUEUE_VALUES {
//         let mut value: size_t = queue_index.wrapping_add(i.wrapping_mul(QUEUES));
//         queue_enqueue(queue, value as *mut libc::c_void);
//         let mut result: libc::c_int = nanosleep(&ENQUEUE_DELAY, 0 as *mut timespec);
//         if result == 0 as libc::c_int {
//         } else {
//             __assert_fail(
//                 b"result == 0\x00" as *const u8 as *const libc::c_char,
//                 b"tests/mqueue_multiple_queues.c\x00" as *const u8 as *const libc::c_char,
//                 21 as libc::c_int as libc::c_uint,
//                 (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
//                     b"void *enqueuer(void *)\x00",
//                 ))
//                 .as_ptr(),
//             );
//         }
//         i = i.wrapping_add(1)
//     }
//     return 0 as *mut libc::c_void;
// }
// #[no_mangle]
// pub unsafe extern "C" fn dequeuer(mut _queue_index: *mut libc::c_void) -> *mut libc::c_void {
//     let mut queue_index: size_t = _queue_index as size_t;
//     let mut queue: *mut queue_t = queues[queue_index as usize];
//     let mut i: size_t = 0 as libc::c_int as size_t;
//     while i < QUEUE_VALUES {
//         let mut value: size_t = queue_dequeue(queue) as size_t;
//         if value == queue_index.wrapping_add(i.wrapping_mul(QUEUES)) {
//         } else {
//             __assert_fail(
//                 b"value == queue_index + i * QUEUES\x00" as *const u8 as *const libc::c_char,
//                 b"tests/mqueue_multiple_queues.c\x00" as *const u8 as *const libc::c_char,
//                 31 as libc::c_int as libc::c_uint,
//                 (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
//                     b"void *dequeuer(void *)\x00",
//                 ))
//                 .as_ptr(),
//             );
//         }
//         i = i.wrapping_add(1)
//     }
//     return 0 as *mut libc::c_void;
// }
// #[test]
// pub fn main() {
//     let mut repetition: size_t = 0 as libc::c_int as size_t;
//     while repetition < REPETITIONS {
//         let vla = QUEUES as usize;
//         let mut enqueue_threads: Vec<pthread_t> = ::std::vec::from_elem(0, vla);
//         let vla_0 = QUEUES as usize;
//         let mut dequeue_threads: Vec<pthread_t> = ::std::vec::from_elem(0, vla_0);
//         let mut i: size_t = 0 as libc::c_int as size_t;
//         while i < QUEUES {
//             queues[i as usize] = queue_init();
//             let mut result: libc::c_int = pthread_create(
//                 &mut *dequeue_threads.as_mut_ptr().offset(i as isize),
//                 0 as *const pthread_attr_t,
//                 Some(dequeuer as unsafe extern "C" fn(_: *mut libc::c_void) -> *mut libc::c_void),
//                 i as *mut libc::c_void,
//             );
//             if result == 0 as libc::c_int {
//             } else {
//                 __assert_fail(
//                     b"result == 0\x00" as *const u8 as *const libc::c_char,
//                     b"tests/mqueue_multiple_queues.c\x00" as *const u8 as *const libc::c_char,
//                     43 as libc::c_int as libc::c_uint,
//                     (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00"))
//                         .as_ptr(),
//                 );
//             }
//             result = pthread_create(
//                 &mut *enqueue_threads.as_mut_ptr().offset(i as isize),
//                 0 as *const pthread_attr_t,
//                 Some(enqueuer as unsafe extern "C" fn(_: *mut libc::c_void) -> *mut libc::c_void),
//                 i as *mut libc::c_void,
//             );
//             if result == 0 as libc::c_int {
//             } else {
//                 __assert_fail(
//                     b"result == 0\x00" as *const u8 as *const libc::c_char,
//                     b"tests/mqueue_multiple_queues.c\x00" as *const u8 as *const libc::c_char,
//                     45 as libc::c_int as libc::c_uint,
//                     (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00"))
//                         .as_ptr(),
//                 );
//             }
//             i = i.wrapping_add(1)
//         }
//         let mut i_0: size_t = 0 as libc::c_int as size_t;
//         while i_0 < QUEUES {
//             let mut result_0: libc::c_int = pthread_join(
//                 *enqueue_threads.as_mut_ptr().offset(i_0 as isize),
//                 0 as *mut *mut libc::c_void,
//             );
//             if result_0 == 0 as libc::c_int {
//             } else {
//                 __assert_fail(
//                     b"result == 0\x00" as *const u8 as *const libc::c_char,
//                     b"tests/mqueue_multiple_queues.c\x00" as *const u8 as *const libc::c_char,
//                     49 as libc::c_int as libc::c_uint,
//                     (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00"))
//                         .as_ptr(),
//                 );
//             }
//             result_0 = pthread_join(
//                 *dequeue_threads.as_mut_ptr().offset(i_0 as isize),
//                 0 as *mut *mut libc::c_void,
//             );
//             if result_0 == 0 as libc::c_int {
//             } else {
//                 __assert_fail(
//                     b"result == 0\x00" as *const u8 as *const libc::c_char,
//                     b"tests/mqueue_multiple_queues.c\x00" as *const u8 as *const libc::c_char,
//                     51 as libc::c_int as libc::c_uint,
//                     (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\x00"))
//                         .as_ptr(),
//                 );
//             }
//             queue_free(queues[i_0 as usize]);
//             i_0 = i_0.wrapping_add(1)
//         }
//         repetition = repetition.wrapping_add(1)
//     }
//     return 0;
// }
