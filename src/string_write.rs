use std::io;
use std::io::{Error, ErrorKind, Write};
use std::mem::size_of;
use encoding::{Encoding, EncoderTrap};
use encoding::all::{UTF_16BE, UTF_16LE};

///
/// Trait that provides various methods to write strings.
/// Automatically implemented for all implementations of io::Write.
/// This trait is sealed and cannot be implemented manually.
///
pub trait StringWrite : private::Sealed {

    ///
    /// Writes an u16 little endian length prefix followed by an utf-8 representation of the string
    /// Returns the total amount of bytes written
    ///
    fn write_string_u16_le_len_utf8(&mut self, string: &str) -> io::Result<usize>;

    ///
    /// Writes an u16 big endian length prefix followed by an utf-8 representation of the string
    /// Returns the total amount of bytes written
    ///
    fn write_string_u16_be_len_utf8(&mut self, string: &str) -> io::Result<usize>;

    ///
    /// Writes an u32 little endian length prefix followed by an utf-8 representation of the string
    /// Returns the total amount of bytes written
    ///
    fn write_string_u32_le_len_utf8(&mut self, string: &str) -> io::Result<usize>;

    ///
    /// Writes an u32 big endian length prefix followed by an utf-8 representation of the string
    /// Returns the total amount of bytes written
    ///
    fn write_string_u32_be_len_utf8(&mut self, string: &str) -> io::Result<usize>;

    ///
    /// Writes an utf-8 representation of the string and a zero byte.
    /// If the string ends with a null char then no zero byte is appended.
    /// If the string contains a null char in the middle then the method fails.
    /// Returns the total amount of bytes written
    ///
    fn write_string_zero_terminated_utf8(&mut self, string: &str) -> io::Result<usize>;

    ///
    /// Writes an utf-8 representation of the string
    /// Returns the total amount of bytes written
    ///
    fn write_string_utf8(&mut self, string: &str) -> io::Result<usize>;

    ///
    /// Writes an utf-16-be representation of the string
    /// Returns the total amount of bytes written
    ///
    fn write_string_utf16_be(&mut self, string: &str) -> io::Result<usize>;

    ///
    /// Writes an utf-16-le representation of the string
    /// Returns the total amount of bytes written
    ///
    fn write_string_utf16_le(&mut self, string: &str) -> io::Result<usize>;

    ///
    /// Writes an utf-32-be representation of the string
    /// Returns the total amount of bytes written
    ///
    fn write_string_utf32_be(&mut self, string: &str) -> io::Result<usize>;

    ///
    /// Writes an utf-32-le representation of the string
    /// Returns the total amount of bytes written
    ///
    fn write_string_utf32_le(&mut self, string: &str) -> io::Result<usize>;

    ///
    /// Writes a string that can be read by a java program using the java.io.DataInput#readUTF facility.
    /// In general, it writes a big endian u16 to indicate how many bytes it will write.
    /// Each character is a re-encoded utf-16 representation of the string.
    /// 1. The string is utf-16 encoded
    /// 2. Each u16 of the utf-16 is encoded as 1,2 or 3 bytes using a custom java specific encoding.
    /// Returns the total amount of bytes written
    ///
    fn write_java_data_output_utf(&mut self, string: &str) -> io::Result<usize>;
}


const ZERO: [u8; 1] = [0u8];
impl <T> StringWrite for T where T: Write {
    fn write_string_u16_le_len_utf8(&mut self, string: &str) -> io::Result<usize> {
        let x = string.as_bytes();
        let len_u16 = x.len() as u16;
        if x.len() != len_u16 as usize {
            return Err(Error::new(ErrorKind::Other, "string too long"));
        }

        self.write_all(len_u16.to_le_bytes().as_slice())?;
        self.write_all(x)?;
        return Ok(x.len()+2);
    }

    fn write_string_u16_be_len_utf8(&mut self, string: &str) -> io::Result<usize> {
        let x = string.as_bytes();
        let len_u16 = x.len() as u16;
        if x.len() != len_u16 as usize {
            return Err(Error::new(ErrorKind::Other, "string too long"));
        }

        self.write_all(len_u16.to_be_bytes().as_slice())?;
        self.write_all(x)?;
        return Ok(x.len()+2);
    }

    fn write_string_u32_le_len_utf8(&mut self, string: &str) -> io::Result<usize> {
        let x = string.as_bytes();
        self.write_all((x.len() as u32).to_le_bytes().as_slice())?;
        self.write_all(x)?;
        return Ok(x.len()+4);
    }

    fn write_string_u32_be_len_utf8(&mut self, string: &str) -> io::Result<usize> {
        let x = string.as_bytes();
        self.write_all((x.len() as u32).to_be_bytes().as_slice())?;
        self.write_all(x)?;
        return Ok(x.len()+4);
    }

    fn write_string_zero_terminated_utf8(&mut self, string: &str) -> io::Result<usize> {
        let x = string.as_bytes();
        if x.len() == 0 {
            return Ok(0);
        }

        for i in 0 .. x.len()-1 {
            if x[i] == 0 {
                return Err(Error::new(ErrorKind::InvalidInput, "Null byte found in string"));
            }
        }

        if x[x.len()-1] == 0 {
            self.write_all(x)?;
            return Ok(x.len());
        }

        self.write_all(x)?;
        self.write_all(&ZERO)?;
        return Ok(x.len()+1);
    }

    fn write_string_utf8(&mut self, string: &str) -> io::Result<usize> {
        let x = string.as_bytes();
        self.write_all(x)?;
        return Ok(x.len());
    }

    fn write_string_utf16_be(&mut self, string: &str) -> io::Result<usize> {
        let encoded = UTF_16BE.encode(string, EncoderTrap::Strict)
            .map_err(|e| Error::new(ErrorKind::Other, e.as_ref()))?;
        self.write_all(encoded.as_slice())?;
        return Ok(encoded.len());
    }

    fn write_string_utf16_le(&mut self, string: &str) -> io::Result<usize> {
        let encoded = UTF_16LE.encode(string, EncoderTrap::Strict)
            .map_err(|e| Error::new(ErrorKind::Other, e.as_ref()))?;
        self.write_all(encoded.as_slice())?;
        return Ok(encoded.len());
    }

    #[cfg(target_endian = "big")]
    fn write_string_utf32_be(&mut self, string: &str) -> io::Result<usize> {
        let mut data: Vec<char> = string.chars().collect();
        let sl : &mut [u8] = unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr().cast(), data.len() * size_of::<char>()) };
        self.write_all(sl)?;
        return Ok(sl.len());
    }

    #[cfg(target_endian = "big")]
    fn write_string_utf32_le(&mut self, string: &str) -> io::Result<usize> {
        let mut data: Vec<u32> = string.chars().map(|c| c as u32).collect();
        for i in 0 .. data.len() {
            data[i] = data[i].to_le();
        }

        let sl : &mut [u8] = unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr().cast(), data.len() * size_of::<u32>()) };
        self.write_all(sl)?;
        return Ok(sl.len());
    }

    #[cfg(target_endian = "little")]
    fn write_string_utf32_be(&mut self, string: &str) -> io::Result<usize> {
        let mut data: Vec<u32> = string.chars().map(|c| c as u32).collect();
        for i in 0 .. data.len() {
            data[i] = data[i].to_be();
        }

        let sl : &mut [u8] = unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr().cast(), data.len() * size_of::<u32>()) };
        self.write_all(sl)?;
        return Ok(sl.len());
    }

    #[cfg(target_endian = "little")]
    fn write_string_utf32_le(&mut self, string: &str) -> io::Result<usize> {
        let mut data: Vec<char> = string.chars().collect();
        let sl : &mut [u8] = unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr().cast(), data.len() * size_of::<char>()) };
        self.write_all(sl)?;
        return Ok(sl.len());
    }

    fn write_java_data_output_utf(&mut self, string: &str) -> io::Result<usize> {
        let encoded = UTF_16LE.encode(string, EncoderTrap::Strict)
            .map_err(|e| Error::new(ErrorKind::Other, e.as_ref()))?;

        if encoded.len() & 1 != 0 {
            panic!("UTF-16 encoder failed and gave a odd number of bytes as output without reporting an error!");
        }

        let sl : &[u16] = unsafe { std::slice::from_raw_parts(encoded.as_ptr().cast(), encoded.len() >> 1) };
        let mut count : usize = 0;
        for c_le in sl {
            let c = c_le.to_le();
            if c < 0x80 && c != 0 {
                count+=1;
                continue;
            }
            if c >= 0x800 {
                count+=3;
                continue;
            }
            count+=2;
        }

        if count > 65535 {
            return Err(Error::new(ErrorKind::Other, "String length exceeds maximum allowed value"));
        }

        let mut data : Vec<u8> = vec![0u8; count+2];
        let mut index = 2usize;
        //This is always big endian in java.
        data[0] = ((count >> 8)& 0xFFusize) as u8;
        data[1] = ((count >> 0)& 0xFFusize) as u8;

        for c_le in sl {
            let c = c_le.to_le();
            if c < 0x80 && c != 0 {
                data[index] = c as u8;
                index+=1;
                continue;
            }

            if c >= 0x800 {
                data[index] = (0xE0 | ((c >> 12) & 0x0F)) as u8;
                data[index+1] = (0x80 | ((c >>  6) & 0x3F)) as u8;
                data[index+2] = (0x80 | ((c >>  0) & 0x3F)) as u8;
                index+=3;
                continue;
            }

            data[index] = (0xC0 | ((c >>  6) & 0x1F)) as u8;
            data[index+1] = (0x80 | ((c >>  0) & 0x3F)) as u8;
            index+=2;
        }

        self.write_all(data.as_slice())?;
        return Ok(data.len());
    }
}

mod private {
    use std::io::Write;

    impl <T> Sealed for T where T: Write {}
    pub trait Sealed {

    }
}
