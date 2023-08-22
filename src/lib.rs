use std::error::Error;
use std::io::{Read, Seek};
pub trait SqliteReader {
    fn generate(&self) -> Result<Box<dyn Read + 'static>, Box<dyn Error>>;
}

pub const READER_POINTER_NAME: &[u8] = b"reader0\0";
pub trait SqliteReaderWithSeek {
    fn generate(&self) -> Result<Box<dyn Seek>, Box<dyn Error>>;
}

pub const READER_SEEK_POINTER_NAME: &[u8] = b"reader_seek0\0";
