use core::ffi::CStr;
use libc::{c_char, c_int, c_ulong, c_void};

use crate::convert_res;

#[no_mangle]
unsafe extern "C" fn mount(
    src: *const c_char,
    target: *const c_char,
    fstype: *const c_char,
    flags: c_ulong,
    data: *const c_void,
) -> c_int {
    libc!(libc::mount(src, target, fstype, flags, data));

    match convert_res(rustix::mount::mount2(
        if src.is_null() {
            None
        } else {
            Some(CStr::from_ptr(src.cast()))
        },
        CStr::from_ptr(target),
        if fstype.is_null() {
            None
        } else {
            Some(CStr::from_ptr(fstype.cast()))
        },
        rustix::mount::MountFlags::from_bits_retain(flags as _),
        if data.is_null() {
            None
        } else {
            Some(CStr::from_ptr(data.cast()))
        },
    )) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn umount(target: *const c_char) -> c_int {
    umount2(target, 0)
}
#[no_mangle]
unsafe extern "C" fn umount2(target: *const c_char, flags: c_int) -> c_int {
    match convert_res(rustix::mount::unmount(
        CStr::from_ptr(target.cast()),
        rustix::mount::UnmountFlags::from_bits_retain(flags as _),
    )) {
        Some(()) => 0,
        None => -1,
    }
}
