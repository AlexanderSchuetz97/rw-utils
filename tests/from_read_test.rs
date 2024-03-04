use std::io;
use std::io::{Cursor, Read, Write};
use rw_utils::from_read::{FromRead};
use rw_utils::num_read::NumRead;
use rw_utils::num_write::NumWrite;
use rw_utils::to_write::ToWrite;


#[derive(Debug, Clone, Default, Eq, PartialEq)]
struct Test {
    a: i32,
    b: i32,
    c: i32,
}

impl FromRead for Test {
    fn copy_from_read(&mut self, mut reader: &mut dyn Read) -> io::Result<()> {
        self.a = reader.read_i32_ne()?;
        self.b = reader.read_i32_ne()?;
        self.c = reader.read_i32_ne()?;
        return Ok(());
    }
}

impl ToWrite for Test {
    fn copy_to_write(&self, mut writer: &mut dyn Write) -> io::Result<()> {
        writer.write_i32_ne(self.a)?;
        writer.write_i32_ne(self.b)?;
        writer.write_i32_ne(self.c)?;
        return Ok(());

    }
}

#[test]
fn simple_test() -> io::Result<()> {
    let mut def = Test::default();
    let mut v : Vec<u8> = vec![];
    def.a = 4;
    def.copy_to_write(&mut v)?;
    def.a = 2;

    let mut cursor = Cursor::new(&v);

    def.copy_from_read(&mut cursor)?;
    assert_eq!(def.a, 4);

    return Ok(());
}