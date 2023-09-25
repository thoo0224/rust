use std::io::Read;

use crate::serialization::Serializable;
use crate::archive::FArchive;
use crate::result::Result;

pub struct FGenericArchive<Reader: Read> {
    reader: Reader
}

impl<Reader: Read> From<Reader> for FGenericArchive<Reader> {
    fn from(value: Reader) -> Self {
        Self {
            reader: value
        }
    }
}

impl<Reader: Read> FArchive for FGenericArchive<Reader> {
    fn read_slice(&mut self, buf: &mut [u8]) -> Result<usize> {
        let read = self.reader.read(buf)?;
        Ok(read)
    }

    fn read<Type: Serializable<Type>>(&mut self) -> Result<Type> {
        let result = Type::deserialize(self)?;
        Ok(result)
    }
}