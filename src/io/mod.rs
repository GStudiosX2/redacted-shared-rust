use core::{fmt::{self, Write}, ffi::c_char};

use alloc::{ffi::CString, string::String};

pub mod input;

unsafe extern "C" {
    #[link_name = "printl"]
    pub unsafe fn ffi_printl(str: *const c_char);
}

#[doc(hidden)]
pub fn puts(str: &str) {
    let cstr = CString::new(str).unwrap();
    unsafe {
        ffi_printl(cstr.as_ptr());
    }
}

#[doc(hidden)]
pub fn putfmt(fmt: fmt::Arguments) -> fmt::Result {
    let mut s = String::new();
    s.write_fmt(fmt).unwrap();
    let cstr = CString::new(s).unwrap();
    unsafe { ffi_printl(cstr.as_ptr()); }
    Ok(())
}

#[doc(hidden)]
pub fn putnl() {
    unsafe {
        ffi_printl(c"".as_ptr());
    }
}