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

#[macro_use]
mod macros;