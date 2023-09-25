use std::io::{Seek, SeekFrom};

use crate::serialization::Serializable;
use crate::archive::FArchive;
use crate::result::Result;

pub struct FPointerArchive {
    pointer: *mut u8,
    pos: i64
}

impl From<*mut u8> for FPointerArchive {
    fn from(value: *mut u8) -> Self {
        Self {
            pointer: value,
            pos: 0
        }
    }
}

impl FArchive for FPointerArchive {
    fn read_slice(&mut self, buf: &mut [u8]) -> Result<usize> {
        let len = buf.len();

        unsafe {
            let ptr = self.pointer.add(self.pos as usize);
            std::ptr::copy_nonoverlapping(ptr, buf.as_mut_ptr(), len);
        }

        self.seek(SeekFrom::Current(len as i64))?;
        Ok(len)
    }

    fn read<Type: Serializable<Type>>(&mut self) -> Result<Type> {
        let result = Type::deserialize(self)?;
        Ok(result)
    }
}

impl Seek for FPointerArchive {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let pos = match pos {
            SeekFrom::Start(val) => val,
            SeekFrom::End(val) => unimplemented!(),
            SeekFrom::Current(val) => (self.pos + val).try_into().unwrap(),
        };

        self.pos = pos as i64;
        println!("Seeking to {pos}");
        Ok(pos)
    }
}