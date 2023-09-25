pub mod memory;
pub mod file;
pub mod pointer;
pub mod generic;

use crate::result::Result;
use crate::serialization::Serializable;

pub trait FArchive {
    fn read_slice(&mut self, buf: &mut [u8]) -> Result<usize>;
    fn read<Type: Serializable<Type>>(&mut self) -> Result<Type>;
}