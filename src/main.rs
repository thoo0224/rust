mod archive;
mod serialization;
mod result;

use std::fs::File;
use std::io::{Cursor, Read, Seek};

use crate::serialization::{Serializable, SizedDeserializer};
use crate::archive::pointer::FPointerArchive;
use crate::archive::FArchive;
use crate::archive::generic::FGenericArchive;
use crate::result::Result;

struct Player {
    points: i32,
    age: i32
}

impl Serializable<Player> for Player {
    fn deserialize<Archive>(archive: &mut Archive) -> Result<Player> where Archive: FArchive {
        let points = archive.read_sized::<i32>()?;
        let age = archive.read_sized::<i32>()?;
        Ok(Player {
            points,
            age
        })
    }
}

fn main() -> Result<()> {
    // let file = File::open("test.bin")?;
    // let mut archive = FFileArchive::from(file);
    // let _69 = archive.read_sized::<i32>()?;

    let mut vec: Vec<u8> = vec![10, 0, 0, 0, 11, 0, 0, 0, 12, 0 ,0 ,0 ];

    // unsafe {
    //     let ptr = vec.as_mut_ptr();
    //     let mut archive = FPointerArchive::from(ptr);
    //
    //     let result = archive.read_sized::<i32>()?;
    //     let result2 = archive.read_sized::<i32>()?;
    //     let result3 = archive.read_sized::<i32>()?;
    //     println!("Result: {result}");
    //     println!("Result2: {result2}");
    //     println!("Result3: {result3}");
    // }

    let cursor = Cursor::new(vec);
    let mut archive = FGenericArchive::from(file);
    let result = archive.read_sized::<i32>()?;
    let result2 = archive.read_sized::<i32>()?;
    let result3 = archive.read_sized::<i32>()?;

    println!("Result: {result}");
    println!("Result2: {result2}");
    println!("Result3: {result3}");


    Ok(())
}
