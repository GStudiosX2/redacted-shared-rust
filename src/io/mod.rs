use core::{ffi::c_char, fmt::{self, Display, Write}};

use alloc::{boxed::Box, ffi::{CString, NulError}, string::String, vec::Vec};

pub mod input;

unsafe extern "C" {
    #[link_name = "printl"]
    pub unsafe fn ffi_printl(str: *const c_char);
}

#[doc(hidden)]
pub fn puts(str: &str) {
    let cstr = CString::new(str).unwrap();
    unsafe {
        ffi_printl(cstr.as_ptr());
    }
}

#[doc(hidden)]
pub fn putfmt(fmt: fmt::Arguments) -> fmt::Result {
    let mut s = String::new();
    s.write_fmt(fmt).unwrap();
    let cstr = CString::new(s).unwrap();
    unsafe { ffi_printl(cstr.as_ptr()); }
    Ok(())
}

#[doc(hidden)]
pub fn putnl() {
    unsafe {
        ffi_printl(c"".as_ptr());
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ErrorKind {
    NotFound,
    NotADirectory,
    IsADirectory,
    ReadOnlyFilesystem,
    InvalidData,
    UnexpectedEof,
    Other,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Self::NotFound => "not found",
            Self::IsADirectory => "is a directory",
            Self::NotADirectory => "not a directory",
            Self::ReadOnlyFilesystem => "readonly filesystem",
            Self::InvalidData => "invalid data",
            Self::UnexpectedEof => "unexpected end of file",
            Self::Other => "other error",
        };
        f.write_str(str)
    }
}

#[derive(Debug)]
enum ErrorData {
    Static(&'static str),
    Error(Box<dyn core::error::Error + Sync + Send>),
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    err: Option<ErrorData>,
}

impl Error {
    pub(crate) const READ_EXACT_EOF: Self = Self::const_new(ErrorKind::UnexpectedEof, "failed to fill whole buffer");
    pub(crate) const INVALID_UTF8: Self = Self::const_new(ErrorKind::InvalidData, "stream did not contain valid UTF-8");

    pub const fn const_new(kind: ErrorKind, err: &'static str) -> Self {
        Self { kind, err: Some(ErrorData::Static(err)) }
    }

    pub fn new(kind: ErrorKind, err: &'static str) -> Self {
        Self { kind, err: Some(ErrorData::Static(err)) }
    }

    pub fn from_error<E: Into<Box<dyn core::error::Error + Sync + Send>>>(kind: ErrorKind, err: E) -> Self {
        Self { kind: kind, err: Some(ErrorData::Error(err.into())) }
    }

    pub fn os_error(kind: ErrorKind) -> Self {
        Self { kind, err: None }
    }

    pub fn other<E: Into<Box<dyn core::error::Error + Sync + Send>>>(err: E) -> Self {
        Self { kind: ErrorKind::Other, err: Some(ErrorData::Error(err.into()))}
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.err {
            Some(ref err) => match err {
                ErrorData::Static(str) if self.kind == ErrorKind::Other => str.fmt(f),
                ErrorData::Static(str) => write!(f, "{}: {}", self.kind, str),
                ErrorData::Error(err) => err.fmt(f),
            },
            None => self.kind.fmt(f),
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(value: ErrorKind) -> Self {
        Self { kind: value, err: None }
    }
}

impl From<NulError> for Error {
    fn from(value: NulError) -> Self {
        Self::from_error(ErrorKind::Other, value)
    }
}

pub type Result<T> = core::result::Result<T, Error>;

pub(crate) fn default_read_to_end<R: Read + ?Sized>(r: &mut R, buf: &mut Vec<u8>) -> Result<usize> {
    let start_len = buf.len();
    loop {
        let mut data = [0u8; 32];
        let read = r.read(&mut data)?;
        if read == 0 {
            return Ok(buf.len() - start_len);
        }

        buf.extend_from_slice(&data[..read]);
    }
}

struct Guard<'a> {
    len: usize,
    buf: &'a mut Vec<u8>
}

impl Drop for Guard<'_> {
    fn drop(&mut self) {
        unsafe { self.buf.set_len(self.len); }
    }
}

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        default_read_to_end(self, buf)
    }

    fn read_to_string(&mut self, buf: &mut String) -> Result<usize> {
        let mut g = Guard { len: buf.len(), buf: unsafe { buf.as_mut_vec() } };
        let ret = default_read_to_end(self, g.buf);

        if str::from_utf8(unsafe { g.buf.get_unchecked(g.len..) }).is_err() {
            ret.and_then(|_| Err(Error::INVALID_UTF8))
        } else {
            g.len = g.buf.len();
            ret
        }
    }

    fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<()> {
        while !buf.is_empty() {
            match self.read(buf) {
                Ok(0) => break,
                Ok(n) => {
                    buf = &mut buf[n..];
                },
                Err(e) => return Err(e),
            }
        }

        if !buf.is_empty() {
            Err(Error::READ_EXACT_EOF)
        } else { 
            Ok(())
        }
    }
}

pub trait Seek {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64>;

    fn stream_position(&mut self) -> Result<u64> {
        self.seek(SeekFrom::Current(0))
    }

    fn seek_relative(&mut self, offset: i64) -> Result<()> {
        self.seek(SeekFrom::Current(offset))?;
        Ok(())
    }

    fn rewind(&mut self) -> Result<()> {
        self.seek(SeekFrom::Start(0))?;
        Ok(())
    }
}