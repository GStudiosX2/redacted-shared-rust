use core::{ffi::c_char, u64};

use alloc::ffi::CString;
use path::Path;

use crate::{io::{self, Error, ErrorKind, Read, Seek, SeekFrom}};

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct FileDescriptor {
    id: u64,
    size: u64,
    cursor: u64,
}

impl FileDescriptor {
    pub unsafe fn from_raw(size: u64, fd_id: u64) -> FileDescriptor {
        FileDescriptor { id: fd_id, size, cursor: 0 }
    }
}

impl FsResult {
    pub fn to_io_error(self) -> Option<Error> {
        match self {
            Self::Success => None,
            Self::NotFound => Some(Error::os_error(ErrorKind::NotFound)),
            Self::DriverError => Some(Error::const_new(ErrorKind::Other, "Driver Error")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum FsResult {
    Success,
    NotFound,
    DriverError,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum SeekType {
    Absolute,
    Relative,
}

fn seek_to_ffi_parts(seek_from: SeekFrom, size: u64) -> (i64, SeekType) {
    match seek_from {
        SeekFrom::Start(start) => (start as i64, SeekType::Absolute),
        SeekFrom::End(end) => (size.checked_add_signed(end).unwrap_or(0) as i64, SeekType::Absolute),
        SeekFrom::Current(current) => (current, SeekType::Relative),
    }
}

unsafe extern "C" {
    #[link_name = "fopen"]
    pub unsafe fn ffi_fopen(path: *const c_char, descriptor: *mut FileDescriptor) -> FsResult;
    #[link_name = "fclose"]
    pub unsafe fn ffi_fclose(descriptor: *const FileDescriptor);
    #[link_name = "seek"]
    pub unsafe fn ffi_fseek(descriptor: *mut FileDescriptor, offset: i64, ty: SeekType);
    #[link_name = "fread"]
    pub unsafe fn ffi_fread(descriptor: *mut FileDescriptor, buf: *mut c_char, size: u64) -> u64;
}

#[derive(Clone)]
pub struct File {
    descriptor: FileDescriptor,
}

impl File {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        unsafe {
            let path = CString::from_vec_unchecked(path.as_ref().as_unix_str().as_bytes().to_vec());
            let mut descriptor = FileDescriptor::default();
            if let Some(err) = ffi_fopen(path.as_ptr(), &mut descriptor as _).to_io_error() {
                return Err(err);
            }
            if descriptor.size == 0 {
                let mut buf = [0u8; 1];
                if ffi_fread(&mut descriptor as _, buf.as_mut_ptr(), 1) == 1 { // virtual unsized files
                    descriptor.size = u64::MAX;
                }
            }
            Ok(Self { descriptor })
        }
    }

    pub fn size(&self) -> usize {
        self.descriptor.size as usize
    }

    pub unsafe fn fd_id(&self) -> u64 {
        self.descriptor.id
    }

    pub fn close(self) {
        unsafe { ffi_fclose(&self.descriptor as _);}
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        unsafe {
            if self.descriptor.size == u64::MAX {
                self.descriptor.cursor = 0;
            }
            if self.descriptor.cursor == self.descriptor.size {
                return Ok(0);
            }
            let mut size = (buf.len() as u64).min(u64::MAX);
            if self.descriptor.cursor + size > self.descriptor.size {
                size = self.descriptor.size - self.descriptor.cursor;
            }
            let ret = ffi_fread(&mut self.descriptor as _, buf.as_ptr() as _, size);
            Ok(ret as usize)
        }
    }
}

impl Seek for File {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        unsafe {
            let (offset, ty) = seek_to_ffi_parts(pos, self.descriptor.size);
            ffi_fseek(&mut self.descriptor as _, offset, ty);
            self.stream_position()
        }
    }
    
    fn stream_position(&mut self) -> io::Result<u64> {
        Ok(self.descriptor.cursor)
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { ffi_fclose(&self.descriptor as _); }
    }
}