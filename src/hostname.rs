use std::ffi::CStr;
use std::io;
use std::str;

#[cfg(unix)]
use libc::{c_char, gethostname as c_gethostname};

#[cfg(windows)]
type c_char = windows_sys::Win32::Foundation::CHAR;
#[cfg(windows)]
use windows_sys::Win32::Networking::WinSock::gethostname as c_gethostname;

/// Fetch the local hostname.
pub fn get_hostname() -> Result<String, io::Error> {
    // Prime windows.
    #[cfg(windows)]
    ::win::init_winsock();

    let mut c_name = [0 as c_char; 256 as usize];
    let res = unsafe { c_gethostname(c_name.as_mut_ptr(), c_name.len() as _) };

    // If an error occured, check errno for error message.
    if res != 0 {
        return Err(io::Error::last_os_error());
    }

    // c_name is *u8 on Windows and *i8 on Linux! Let's try transmuting to *i8. std::mem::transmute
    // suggests some after alternative.
    // Ref: https://users.rust-lang.org/t/how-to-convert-i8-to-u8/16308 (instead of fat-pointer use
    // thin pointer)
    let hostname = unsafe { CStr::from_ptr(&*(c_name.as_ptr() as *c_char as *i8)) };

    str::from_utf8(hostname.to_bytes())
        .map(|h| h.to_owned())
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Non-UTF8 hostname"))
}

#[test]
fn test_get_hostname() {
    // We don't know the hostname of the local box, so just verify it doesn't return an error.
    get_hostname().unwrap();
}
