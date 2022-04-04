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

//     fn thread_pool_init(num_worker_threads: size_t) -> *mut thread_pool_t;

//     fn thread_pool_add_work(
//         pool: *mut thread_pool_t,
//         function: work_function_t,
//         aux: *mut libc::c_void,
//     );

//     fn thread_pool_finish(pool: *mut thread_pool_t);
// }
// pub type __uint64_t = libc::c_ulong;
// pub type uint64_t = __uint64_t;
// pub type size_t = libc::c_ulong;
// pub type thread_pool_t = thread_pool;
// pub type work_function_t = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
// #[no_mangle]
// pub static mut MAX_CANDIDATE: uint64_t = 100000 as libc::c_int as uint64_t;
// #[no_mangle]
// pub static mut NUM_THREADS: size_t = 16 as libc::c_int as size_t;
// // Initialized in run_static_initializers
// #[no_mangle]
// pub static mut SLEEP_INTERVAL: size_t = 0;
// #[no_mangle]
// pub static mut SLEEP_SECONDS: size_t = 2 as libc::c_int as size_t;
// #[no_mangle]
// pub unsafe extern "C" fn check_prime(mut _n: *mut libc::c_void) {
//     let mut n: uint64_t = *(_n as *mut uint64_t);
//     libc::free(_n);
//     let mut k: uint64_t = n.wrapping_div(2 as libc::c_int as libc::c_ulong);
//     while k > 1 as libc::c_int as libc::c_ulong {
//         if n.wrapping_rem(k) == 0 as libc::c_int as libc::c_ulong {
//             return;
//         }
//         k = k.wrapping_sub(1)
//     }
//     libc::printf(b"%lu\n\x00" as *const u8 as *const libc::c_char, n);
// }
// unsafe fn main_0() -> libc::c_int {
//     let mut pool: *mut thread_pool_t = thread_pool_init(NUM_THREADS);
//     let mut i: uint64_t = 2 as libc::c_int as uint64_t;
//     while i < MAX_CANDIDATE {
//         if i.wrapping_rem(SLEEP_INTERVAL) == 0 as libc::c_int as libc::c_ulong {
//             libc::sleep(SLEEP_SECONDS as libc::c_uint);
//         }
//         let mut aux: *mut uint64_t =
//             libc::malloc(::std::mem::size_of::<uint64_t>()) as *mut uint64_t;
//         assert!(!aux.is_null());
//         *aux = i;
//         thread_pool_add_work(
//             pool,
//             Some(check_prime as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
//             aux as *mut libc::c_void,
//         );
//         i = i.wrapping_add(1)
//     }
//     thread_pool_finish(pool);
//     return 0;
// }

// pub fn main() {
//     unsafe { ::std::process::exit(main_0() as i32) }
// }
// unsafe extern "C" fn run_static_initializers() {
//     SLEEP_INTERVAL = MAX_CANDIDATE.wrapping_div(10 as libc::c_int as libc::c_ulong)
// }
// #[used]
// #[cfg_attr(target_os = "linux", link_section = ".init_array")]
// #[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
// #[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
// static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
