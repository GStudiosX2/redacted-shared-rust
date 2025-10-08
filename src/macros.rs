#[macro_export]
macro_rules! println {
    () => { $crate::io::putnl(); };
    ($($arg:tt)*) => {
        $crate::io::putfmt(format_args!($($arg)*)).unwrap();
    }
}