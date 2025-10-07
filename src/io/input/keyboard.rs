use crate::io::input::keycodes::{Key, ModifierKey};

#[derive(Clone, Copy, Default, Debug)]
#[repr(C)]
pub struct Keypress {
    pub modifier: ModifierKey,
    rsvd: u8,
    pub keys: [Key; 6],
}

#[derive(Clone, Copy, Default, Debug)]
#[repr(C)]
pub enum KeyEventType {
    #[default]
    KeyRelease,
    KeyPress,
    ModRelease,
    ModPress,
}

#[derive(Clone, Copy, Default, Debug)]
#[repr(C)]
pub struct KeyEvent {
    pub ty: KeyEventType,
    pub key: Key,
    pub modifier: ModifierKey,
}

unsafe extern "C" {
    #[link_name = "read_key"]
    pub unsafe fn ffi_read_key(kp: *mut Keypress) -> bool;
    #[link_name = "read_event"]
    pub unsafe fn ffi_read_event(event: *mut KeyEvent) -> bool;
    #[link_name = "hid_to_char"]
    pub unsafe fn ffi_hid_to_char(hid: u8) -> u8;
}

pub fn hid_to_char(hid: u8) -> char {
    let c = unsafe { ffi_hid_to_char(hid) };
    if c == 0 {
        '?'
    } else {
        c as char
    }
}

pub fn read_key() -> Option<Keypress> {
    let mut keypress = Keypress::default();
    if unsafe { ffi_read_key(&mut keypress) } {
        Some(keypress)
    } else {
        None
    }
}

pub fn read_event() -> Option<KeyEvent> {
    let mut event = KeyEvent::default();
    if unsafe { ffi_read_event(&mut event) } {
        Some(event)
    } else {
        None
    }
}