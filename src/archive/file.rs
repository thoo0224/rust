use std::fs::File;
use std::io::{BufReader, Read};

use crate::serialization::Serializable;
use crate::archive::FArchive;
use crate::Result;

pub struct FFileArchive {
    reader: BufReader<File>,
    length: i64
}

impl From<File> for FFileArchive {
    fn from(value: File) -> Self {
        let length = value.metadata().unwrap().len() as i64;
        let reader = BufReader::new(value);

        Self {
            reader,
            length
        }
    }
}

impl FArchive for FFileArchive {
    fn read_slice(&mut self, buf: &mut [u8]) -> Result<usize> {
        let read = self.reader.read(buf)?;
        Ok(read)
    }

    fn read<Type: Serializable<Type>>(&mut self) -> Result<Type> {
        let result = Type::deserialize(self)?;
        Ok(result)
    }
}