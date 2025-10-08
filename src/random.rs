use crate::{fs::File, io::{Read, Result}};

pub struct Random<R: Read + 'static> {
    source: R,
}

pub unsafe fn random_source() -> File {
    File::open("/random").unwrap()
}

impl Random<File> {
    pub fn new() -> Self {
        Self { source: unsafe { random_source() } }
    }
}

impl<R: Read> Random<R> {
    pub unsafe fn with_source(source: R) -> Self {
        Self { source }
    }

    pub fn fill_buf(&mut self, buf: &mut [u8]) -> Result<()> {
        self.source.read(buf)?;
        Ok(())
    }

    pub fn next_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.fill_buf(&mut buf)?;
        Ok(buf[0])
    }

    pub fn next_u16(&mut self) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.fill_buf(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }

    pub fn next_u32(&mut self) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.fill_buf(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    pub fn next_u64(&mut self) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.fill_buf(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }

    pub fn next_u128(&mut self) -> Result<u128> {
        let mut buf = [0u8; 16];
        self.fill_buf(&mut buf)?;
        Ok(u128::from_le_bytes(buf))
    }
}