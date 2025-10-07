use core::fmt::Display;

use alloc::vec::Vec;

use crate::backtrace::sys::{trace, FrameRecord};

mod sys;

#[derive(Debug)]
pub struct Backtrace {
    pub frames: Vec<TraceFrame>,
    pub start: usize,
}

#[derive(Debug)]
pub struct TraceFrame {
    pub frame: FrameRecord,
}

impl Backtrace {
    #[inline(never)]
    pub fn new() -> Self {
        Self::create(Self::new as usize)
    }

    fn create(ip: usize) -> Backtrace {
        let mut frames = Vec::new();
        let mut actual_start = None;

        trace(&mut |fr| {
            frames.push(TraceFrame {
                frame: *fr,
            });

            if fr.return_addr == ip && actual_start.is_none() {
                actual_start = Some(frames.len());
            }

            true
        }, 100);

        Backtrace { frames, start: actual_start.unwrap_or(0) }
    }
}

impl Display for Backtrace {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let frames = &self.frames[self.start..];

        writeln!(f, "stack backtrace:")?;
        for i in 0..frames.len() {
            writeln!(f, "  {}: {:#x}", i + 1, frames[i].frame.return_addr)?;
        }

        Ok(())
    }
}