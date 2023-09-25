use std::io::{SeekFrom, Seek};

use crate::serialization::Serializable;
use crate::archive::FArchive;
use crate::result::Result;

struct FMemoryArchive {
    data: Vec<u8>,
    length: i64,
    pos: u64
}

impl From<Vec<u8>> for FMemoryArchive {
    fn from(value: Vec<u8>) -> Self {
        let length = value.len() as i64;

        Self {
            data: value,
            length,
            pos: 0
        }
    }
}

impl FArchive for FMemoryArchive {
    fn read_slice(&mut self, buf: &mut [u8]) -> Result<usize> {
        let len = buf.len();
        let pos = self.pos as usize;
        let data = &self.data[pos..pos+len];
        buf.copy_from_slice(data);

        self.seek(SeekFrom::Current(len as i64))?;
        Ok(len)
    }

    fn read<Type: Serializable<Type>>(&mut self) -> Result<Type> {
        let result = Type::deserialize(self)?;
        Ok(result)
    }
}

impl Seek for FMemoryArchive {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let pos = match pos {
            SeekFrom::Start(offset) => offset,
            SeekFrom::Current(offset) => (self.pos+offset as u64),
            SeekFrom::End(offset) => (self.length+offset) as u64
        };

        self.pos = pos;
        Ok(pos)
    }
}