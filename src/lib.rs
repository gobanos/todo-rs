extern crate libc;
use libc::{c_char, uint32_t};
use std::ffi::CStr;
use std::str;

pub mod task_list;

pub use task_list::*;

#[no_mangle]
pub extern fn task_list_new(name: *const c_char) -> *mut TaskList {
    let name = unsafe {
        assert!(!name.is_null());

        CStr::from_ptr(name)
    };

    let name = name.to_str().unwrap().clone();

    Box::into_raw(Box::new(TaskList::new(name.to_string())))
}

#[no_mangle]
pub extern fn task_list_free(list: *mut TaskList) {
    if list.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(list);
    }
}

#[no_mangle]
pub extern fn task_list_add(list: *mut TaskList, name: *const c_char) {
    let list = unsafe {
        assert!(!list.is_null());

        &mut *list
    };

    let name = unsafe {
        assert!(!name.is_null());

        CStr::from_ptr(name)
    };

    let name = name.to_str().unwrap().clone();

    list.add(name.to_string());
}

#[no_mangle]
pub extern fn task_list_remove(list: *mut TaskList, num: uint32_t) {
    let list = unsafe {
        assert!(!list.is_null());

        &mut *list
    };

    let num = num as usize;

    list.remove(num);
}

#[no_mangle]
pub extern fn task_list_toggle(list: *mut TaskList, num: uint32_t) {
    let list = unsafe {
        assert!(!list.is_null());

        &mut *list
    };

    let num = num as usize;

    list.toggle(num);
}