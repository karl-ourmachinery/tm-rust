#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ptr;
use std::ffi::CString;
use core::ffi::c_void;
use std::os::raw::c_char;

static mut tm_logger_api: *mut tm_logger_api = ptr::null_mut();

const file: &'static [u8] = b"Hello from Rust\0";

unsafe extern "C"  fn start(args: *mut tm_simulate_start_args_t) -> *mut tm_simulate_state_o {
    let a = *(*args).allocator;
    let state: *mut tm_simulate_state_o = ptr::null_mut();
    a.realloc.map(|r| r((*args).allocator, state as *mut c_void, 0, std::mem::size_of::<tm_simulate_state_o>() as u64, file.as_ptr() as *const i8, 0));
    return state;
}

unsafe extern "C" fn tick(state: *mut tm_simulate_state_o, args: *mut tm_simulate_frame_args_t) {
    (*tm_logger_api).print.map(|f| f(tm_log_type_TM_LOG_TYPE_INFO, file.as_ptr() as *const i8));
}

const name: &'static [u8] = b"Rusty Game\0";

static mut simulate_entry_i: tm_simulate_entry_i = tm_simulate_entry_i {
    id: tm_strhash_t { u64: 7 },
    display_name: 0 as *const c_char,
    start: Some(start),
    tick: Some(tick),
    hot_reload: None,
    stop: None
};

#[no_mangle]
pub unsafe extern "C" fn tm_load_plugin(reg: *mut tm_api_registry_api, load: bool) {
    simulate_entry_i.display_name = name.as_ptr() as *const i8;
    if load {
        (*reg).add_implementation.map(|f| f(TM_SIMULATE_ENTRY_INTERFACE_NAME.as_ptr() as *const i8, &mut simulate_entry_i as *mut _ as *mut c_void));
    } else {
        (*reg).remove_implementation.map(|f| f(TM_SIMULATE_ENTRY_INTERFACE_NAME.as_ptr() as *const i8, &mut simulate_entry_i  as *mut _ as *mut c_void));
    }

    tm_logger_api = (*reg).get.map(|f| f(TM_LOGGER_API_NAME.as_ptr() as *const i8)).expect("failed looking up logger") as  *mut tm_logger_api;
}