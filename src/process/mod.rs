use core::{convert::Infallible, fmt};

use crate::println;

unsafe extern "C" {
    #[link_name = "halt"]
    pub unsafe fn ffi_exit(exit_code: u32) -> !;
}

pub enum ExitCode {
    SUCCESS = 0,
    FAILURE = 1,
}

impl ExitCode {
    pub fn exit_process(self) -> ! {
        unsafe { ffi_exit(self as u32); }
    }
}

#[lang = "termination"]
#[diagnostic::on_unimplemented(
    message = "`main` has invalid return type `{Self}`",
    label = "`main` can only return types that implement `{Self}`"
)]
pub trait Termination {
    fn report(self) -> ExitCode;   
}

impl Termination for () {
    fn report(self) -> ExitCode {
        ExitCode::SUCCESS
    }
}

impl Termination for ! {
    fn report(self) -> ExitCode {
        self
    }
}

impl Termination for Infallible {
    fn report(self) -> ExitCode {
        match self {}
    }
}

impl Termination for ExitCode {
    fn report(self) -> ExitCode {
        self
    }
}

impl<T: Termination, E: fmt::Debug> Termination for Result<T, E> {
    fn report(self) -> ExitCode {
        match self {
            Ok(val) => val.report(),
            Err(e) => {
                println!("Error: {:#?}", e);
                ExitCode::FAILURE
            }
        }
    }
}