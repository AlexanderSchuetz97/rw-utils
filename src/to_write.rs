use std::io;
use std::io::Write;

///
/// This trait can be used by all structs that can be serialized to a Write.
///
/// I would not recommend using this if you have control over the serialization and
/// only want to use this for rust<->rust serialization. Serde is better for this purpose.
/// This is useful for use cases where precise control of how the data is written is required.
///
/// Example: Image Encoding, Network Packet encoding,...
///
pub trait ToWrite {
    fn copy_to_write(&self, writer:  &mut dyn Write) -> io::Result<()>;
}

///
/// This trait is automatically implemented for all ToWrite impls to allow a
/// struct to be copied into a Vec<u8>.
/// This trait is sealed and cannot be implemented manually.
///
pub trait ToVec: private::Sealed {
    fn copy_to_vec(&self) -> io::Result<Vec<u8>>;
}



impl <T> ToVec for T where T: ToWrite {
    fn copy_to_vec(&self) -> io::Result<Vec<u8>> {
        let mut data = Vec::with_capacity(1024);
        self.copy_to_write(&mut data)?;
        return Ok(data);
    }
}

mod private {
    use crate::to_write::ToWrite;

    impl <T> Sealed for T where T: ToWrite {}
    pub trait Sealed {

    }
}


