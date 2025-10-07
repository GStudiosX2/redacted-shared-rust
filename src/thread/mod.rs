use crate::time::Duration;

unsafe extern "C" {
    #[link_name = "sleep"]
    pub unsafe fn ffi_sleep_ms(ms: u64);
}

pub fn sleep(dur: Duration) {
    let ms = dur.as_millis() as u64;
    unsafe {
        ffi_sleep_ms(ms);
    }
}