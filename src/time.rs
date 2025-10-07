use core::ops::{Add, AddAssign, Sub, SubAssign};
pub use core::time::Duration;

unsafe extern "C" {
    #[link_name = "get_time"]
    pub unsafe fn ffi_get_time_ms() -> u64;
}

#[derive(Debug, Clone, Copy)]
pub struct SystemTime(u64);

impl SystemTime {
    pub fn now() -> Self {
        Self(unsafe {
            ffi_get_time_ms()
        })
    }

    pub fn duration_since(&self, earlier: SystemTime) -> Duration {
        if self.0 >= earlier.0 {
            Duration::from_millis(self.0 - earlier.0)
        } else {
            earlier.duration_since(*self)
        }
    }

    pub fn elapsed(&self) -> Duration {
        SystemTime::now().duration_since(*self)
    }

    pub fn checked_add(&self, duration: Duration) -> Option<SystemTime> {
        self.0.checked_add(duration.as_millis() as u64).map(SystemTime)
    }

    pub fn checked_sub(&self, duration: Duration) -> Option<SystemTime> {
        self.0.checked_sub(duration.as_millis() as u64).map(SystemTime)
    }
}

impl Add<Duration> for SystemTime {
    type Output = SystemTime;

    fn add(self, rhs: Duration) -> Self::Output {
        self.checked_add(rhs).expect("overflowing when adding duration")
    }
}

impl AddAssign<Duration> for SystemTime {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl Sub<Duration> for SystemTime {
    type Output = SystemTime;

    fn sub(self, rhs: Duration) -> Self::Output {
        self.checked_sub(rhs).expect("overflowing when subtracting duration")
    }
}

impl SubAssign<Duration> for SystemTime {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}