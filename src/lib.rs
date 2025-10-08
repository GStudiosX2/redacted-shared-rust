#![no_std]
#![allow(internal_features)]
#![feature(lang_items, never_type)]

pub extern crate core;
pub extern crate alloc;

#[doc(hidden)]
#[macro_use]
pub mod rt;
pub mod io;
pub mod allocator;
pub mod process;
/* RedactedOS doesn't have the concept of threads yet but im trying to build ruststd-like API */
pub mod thread;
pub mod time;
pub extern crate unix_path as path;
pub mod fs;

pub mod backtrace;

#[macro_use]
mod macros;