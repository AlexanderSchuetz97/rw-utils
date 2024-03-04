use std::io;
use std::io::{Cursor, Read};

///
/// This trait can be used by all structs that can be serialized to a Read.
///
/// I would not recommend using this if you have control over the serialization and
/// only want to use this for rust<->rust serialization. Serde is better for this purpose.
/// This is useful for use cases where precise control of how the data is read is required.
///
/// Example: Image Decoding, Network Packet decoding,...
///
pub trait FromRead {
    fn copy_from_read(&mut self, reader: &mut dyn Read) -> io::Result<()>;
}

///
/// Auto implemented trait to add constructors to all structs that provide a Default impl.
/// This trait is sealed and cannot be implemented manually.
///
pub trait DefaultFromRead : Sized + private::Sealed {
    fn from_io(read: &mut dyn Read) -> io::Result<Self>;

    fn from_vec(vec: &Vec<u8>) -> io::Result<Self>;

    fn from_slice(slice: &[u8]) -> io::Result<Self>;
}

impl <T> DefaultFromRead for T where T: FromRead + Default {
    fn from_io(read: &mut dyn Read) -> io::Result<Self> {
        let mut x = Self::default();
        x.copy_from_read(read)?;
        return Ok(x);
    }
    fn from_vec(vec: &Vec<u8>) -> io::Result<Self> {
        let mut x:Cursor<&[u8]> = Cursor::new(vec.as_ref());
        return Self::from_io(&mut x);
    }
    fn from_slice(slice: &[u8]) -> io::Result<Self> {
        let mut x:Cursor<&[u8]> = Cursor::new(slice.as_ref());
        return Self::from_io(&mut x);
    }
}

mod private {
    use crate::from_read::FromRead;

    impl <T> Sealed for T where T: FromRead + Default {}
    pub trait Sealed {

    }
}


mod test {

}
