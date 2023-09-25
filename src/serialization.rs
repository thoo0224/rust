use std::mem::MaybeUninit;

use crate::archive::FArchive;
use crate::result::Result;

pub trait Serializable<T> {
    fn deserialize<Archive>(archive: &mut Archive) -> Result<T> where Archive: FArchive;
}

pub trait SizedDeserializer<Archive: FArchive> {
    fn read_sized<Type: Sized>(&mut self) -> Result<Type>;
}

impl<Archive: FArchive> SizedDeserializer<Archive> for Archive {
    fn read_sized<Type>(&mut self) -> Result<Type> {
        let mut result: MaybeUninit<Type> = MaybeUninit::uninit();
        let size = std::mem::size_of::<Type>();

        let mut buf = vec![0u8; size];
        self.read_slice(&mut buf)?;

        unsafe {
            let dest = result.as_mut_ptr() as *mut u8;
            let src = buf.as_ptr();
            std::ptr::copy_nonoverlapping(src, dest, size);

            Ok(result.assume_init())
        }
    }
}
