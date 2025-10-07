use core::{fmt::{self, Write}, ffi::c_char};

use alloc::{ffi::CString, vec::Vec};

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
    struct FmtBuf(Vec<u8>);

    impl fmt::Write for FmtBuf {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.extend(s.as_bytes());
            Ok(())
        }
    }

    let mut buf = FmtBuf(Vec::with_capacity(16));
    buf.write_fmt(fmt)?;
    let cstr = CString::new(buf.0).unwrap();
    unsafe { ffi_printl(cstr.as_ptr()); }
    Ok(())
}

#[doc(hidden)]
pub fn putnl() {
    unsafe {
        ffi_printl(c"".as_ptr());
    }
}