use core::panic::PanicInfo;

use crate::{println, process::{self, Termination}};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    process::ExitCode::FAILURE.exit_process()
}

#[lang = "start"]
fn lang_start<T: Termination + 'static>(
    main: fn() -> T,
    _argc: isize,
    _argv: *const *const u8,
    _: u8
) -> isize {
    // TODO: argc, argv though i don't think RedactedOS has a way to provide those yet
    main().report() as isize
}