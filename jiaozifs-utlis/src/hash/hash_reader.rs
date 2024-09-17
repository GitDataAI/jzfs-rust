use std::io::{self, Read, Write};
use sha2::{Sha256, Digest};
use md5::Md5;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use crate::Hash;

#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum HashType {
    Md5 = 0,
    SHA256 = 1,
}

#[derive(Clone)]
pub struct Hasher {
    pub md5: Option<Md5>,
    pub sha256: Option<Sha256>,
}

impl Hasher {
    pub fn new(hash_types: &[HashType]) -> Self {
        let mut hasher = Hasher {
            md5: None,
            sha256: None,
        };
        for &hash_type in hash_types {
            match hash_type {
                HashType::Md5 => {
                    if hasher.md5.is_none() {
                        hasher.md5 = Some(Md5::new());
                    }
                }
                HashType::SHA256 => {
                    if hasher.sha256.is_none() {
                        hasher.sha256 = Some(Sha256::new());
                    }
                }
            }
        }
        hasher
    }
    pub fn to_md5_hash(&self) -> io::Result<Hash> {
        Ok(Hash(self.clone().md5.unwrap().decompose().1.get_data().to_vec()))
    }
    pub fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        if let Some(ref mut md5) = self.md5 {
            md5.write_all(data)?;
        }
        if let Some(ref mut sha256) = self.sha256 {
            sha256.write_all(data)?;
        }
        Ok(data.len())
    }

    pub fn write_int8(&mut self, data: i8) -> io::Result<usize> {
        self.write(&[data as u8])
    }

    pub fn write_uint8(&mut self, data: u8) -> io::Result<usize> {
        self.write(&[data])
    }

    pub fn write_string(&mut self, data: &str) -> io::Result<usize> {
        self.write(data.as_bytes())
    }

    pub fn write_int32(&mut self, data: i32) -> io::Result<usize> {
        let mut buf = [0u8; 4];
        buf.copy_from_slice(&data.to_be_bytes());
        self.write(&buf)
    }

    pub fn write_uint32(&mut self, data: u32) -> io::Result<usize> {
        let mut buf = [0u8; 4];
        buf.copy_from_slice(&data.to_be_bytes());
        self.write(&buf)
    }

    pub fn write_int64(&mut self, data: i64) -> io::Result<usize> {
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&data.to_be_bytes());
        self.write(&buf)
    }

    pub fn write_uint64(&mut self, data: u64) -> io::Result<usize> {
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&data.to_be_bytes());
        self.write(&buf)
    }

}

pub struct HashingReader<R: Read> {
    hasher: Hasher,
    original_reader: R,
    copied_size: u64,
}

impl<R: Read> HashingReader<R> {
    pub fn new(reader: R, hash_types: &[HashType]) -> Self {
        HashingReader {
            hasher: Hasher::new(hash_types),
            original_reader: reader,
            copied_size: 0,
        }
    }
}

impl<R: Read> Read for HashingReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let nb = self.original_reader.read(buf)?;
        self.copied_size += nb as u64;
        if let Some(ref mut md5) = self.hasher.md5 {
            md5.write_all(&buf[..nb])?;
        }
        if let Some(ref mut sha256) = self.hasher.sha256 {
            sha256.write_all(&buf[..nb])?;
        }
        Ok(nb)
    }
}


#[cfg(test)]
mod hash_reader_test{
    use super::*;
    #[test]
    fn test_hash_writer(){
        let mut writer = Hasher::new(&[HashType::Md5, HashType::SHA256]);
        assert!(writer.write_int8(1).is_ok());
        assert!(writer.write_uint8(2).is_ok());
        assert!(writer.write_string("hello").is_ok());
        assert!(writer.write_int32(3).is_ok());
        assert!(writer.write_uint32(4).is_ok());
        assert!(writer.write_int64(5).is_ok());
        assert!(writer.write_uint64(6).is_ok());
    }
    #[test]
    fn test_hashing_reader(){
        let data = b"hello world";
        let mut reader = HashingReader::new(&data[..], &[HashType::Md5, HashType::SHA256]);
        let mut buf = [0u8; 5];
        assert!(reader.read(&mut buf).is_ok());
    }
}