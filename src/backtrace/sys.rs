use core::arch::asm;


struct Bomb { enabled: bool }

impl Drop for Bomb {
    fn drop(&mut self) {
        if self.enabled {
            panic!("can't panic during backtrace");
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct FrameRecord {
    pub prev: *const FrameRecord,
    pub return_addr: usize
}

impl FrameRecord {
    pub fn current() -> *const Self {
        unsafe { 
            let x29: *const Self;
            asm!("mov {0}, x29", out(reg) x29, options(nostack));
            x29
        }
    }
}

pub fn trace(cb: &mut dyn FnMut(&FrameRecord) -> bool, depth: usize) {
    unsafe {
        let mut fp = FrameRecord::current();

        for _ in 0..depth {
            if fp.is_null() { break; }
    
            let fr = &*fp;
            if fr.return_addr == 0 || fr.prev.is_null() {
                break;
            }

            let mut bomb = Bomb { enabled: true };
            cb(fr);
            bomb.enabled = false;

            fp = fr.prev;
        }
    }
}