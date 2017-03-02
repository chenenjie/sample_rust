extern crate byteorder;


use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use byteorder::{LittleEndian, WriteBytesExt};


fn main() {
    let mut rdr = Cursor::new(vec![2, 5, 3, 0]);


    assert_eq!(517, rdr.read_u16::<BigEndian>().unwrap());
    assert_eq!(768, rdr.read_u16::<BigEndian>().unwrap());


    let mut wtr = vec![];
    wtr.write_u16::<LittleEndian>(517).unwrap();
    wtr.write_u16::<LittleEndian>(768).unwrap();
    assert_eq!(wtr, vec![5, 2, 0, 3]);
}