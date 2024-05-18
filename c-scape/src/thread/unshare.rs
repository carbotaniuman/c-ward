use libc::c_int;

use crate::convert_res;

#[no_mangle]
unsafe extern "C" fn unshare(flags: c_int) -> c_int {
    libc!(libc::unshare(flags));

    match convert_res(rustix::thread::unshare(
        rustix::thread::UnshareFlags::from_bits_retain(flags as _),
    )) {
        Some(()) => 0,
        None => -1,
    }
}
