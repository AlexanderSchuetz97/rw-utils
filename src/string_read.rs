use std::io;
use std::io::{Error, ErrorKind, Read};
use std::mem::size_of;

///
/// Trait that provides various methods to read strings.
/// Automatically implemented for all implementations of io::Read.
/// This trait is sealed and cannot be implemented manually.
///
pub trait StringRead : private::Sealed {

    ///
    /// Reads an u32 in little endian length prefix followed by that amount of bytes.
    /// The bytes are then parsed as utf-8.
    ///
    fn read_string_u16_le_len_utf8(&mut self) -> io::Result<String>;

    ///
    /// Reads an u32 in big endian length prefix followed by that amount of bytes.
    /// The bytes are then parsed as utf-8.
    ///
    fn read_string_u16_be_len_utf8(&mut self) -> io::Result<String>;

    ///
    /// Reads an u32 in little endian length prefix followed by that amount of bytes.
    /// The bytes are then parsed as utf-8.
    ///
    fn read_string_u32_le_len_utf8(&mut self) -> io::Result<String>;

    ///
    /// Reads an u32 in big endian length prefix followed by that amount of bytes.
    /// The bytes are then parsed as utf-8.
    ///
    fn read_string_u32_be_len_utf8(&mut self) -> io::Result<String>;

    ///
    /// Reads until zero byte and treats all bytes read as utf-8 string.
    ///
    fn read_string_zero_terminated_utf8(&mut self) -> io::Result<String>;

    ///
    /// Read given amount of bytes and treat them as UTF-8 string.
    ///
    fn read_string_utf8(&mut self, size: usize) -> io::Result<String>;

    ///
    /// Read given amount of characters of an utf-16-be string.
    ///
    fn read_string_utf16_be(&mut self, size_in_characters: usize) -> io::Result<String>;

    ///
    /// Read given amount of characters of an utf-16-le string.
    ///
    fn read_string_utf16_le(&mut self, size_in_characters: usize) -> io::Result<String>;

    ///
    /// Read given amount of characters of an utf-32-be string.
    ///
    fn read_string_utf32_be(&mut self, size_in_characters: usize) -> io::Result<String>;

    ///
    /// Read given amount of characters of an utf-32-le string.
    ///
    fn read_string_utf32_le(&mut self, size_in_characters: usize) -> io::Result<String>;

    ///
    /// Reads a string that was produced by a java program using the java.io.DataOutput#writeUTF facility.
    /// In general, it reads an u16 in big endian to indicate how many further bytes are needed.
    /// It then parses the data to create an utf-16 u16 array.
    /// Each u16 consists of 1, 2 or 3 bytes encoded in a custom java specific encoding.
    /// After parsing all the data the u16 array contains utf-16
    /// data that will be turned into a String using String::from_utf16.
    ///
    fn read_java_data_input_utf(&mut self) -> io::Result<String>;
}

impl <T> StringRead for T where T: Read {
    fn read_string_u16_le_len_utf8(&mut self) -> io::Result<String> {
        let mut len_bytes = [0u8; 2];
        self.read_exact(len_bytes.as_mut_slice())?;
        let len = u16::from_le_bytes(len_bytes);
        return self.read_string_utf8(len as usize);
    }

    fn read_string_u16_be_len_utf8(&mut self) -> io::Result<String> {
        let mut len_bytes = [0u8; 2];
        self.read_exact(len_bytes.as_mut_slice())?;
        let len = u16::from_be_bytes(len_bytes);
        return self.read_string_utf8(len as usize);
    }

    fn read_string_u32_le_len_utf8(&mut self) -> io::Result<String> {
        let mut len_bytes = [0u8; 4];
        self.read_exact(len_bytes.as_mut_slice())?;
        let len = u32::from_le_bytes(len_bytes);
        return self.read_string_utf8(len as usize);
    }

    fn read_string_u32_be_len_utf8(&mut self) -> io::Result<String> {
        let mut len_bytes = [0u8; 4];
        self.read_exact(len_bytes.as_mut_slice())?;
        let len = u32::from_be_bytes(len_bytes);
        return self.read_string_utf8(len as usize);
    }

    fn read_string_zero_terminated_utf8(&mut self) -> io::Result<String> {
        let mut data = Vec::with_capacity(64);
        let mut buf = [0u8];
        let sl = buf.as_mut_slice();
        loop {
            self.read_exact(sl)?;
            if sl[0] == 0 {
                break;
            }

            data.push(sl[0]);
        }

        return String::from_utf8(data).map_err(|_e| Error::new(ErrorKind::InvalidData, "invalid utf-8 data"));
    }

    fn read_string_utf8(&mut self, size: usize) -> io::Result<String> {
        let mut data = vec![0u8; size];
        self.read_exact(data.as_mut_slice())?;
        return String::from_utf8(data).map_err(|_e| Error::new(ErrorKind::InvalidData, "invalid utf-8 data"));
    }

    #[cfg(target_endian = "little")]
    fn read_string_utf16_be(&mut self, size_in_characters: usize) -> io::Result<String> {
        if size_in_characters == 0 {
            return Ok("".to_string());
        }

        let mut data = vec![0u8; size_in_characters<<1];
        self.read_exact(data.as_mut_slice())?;

        let sl :&mut [u16] = unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr().cast(), size_in_characters) };
        for i in 0 .. sl.len() {
            sl[i] = sl[i].to_be();
        }

        if sl[0] == 0xFFFE {
            return Err(Error::new(ErrorKind::InvalidData, "Encountered byte order mark 0xFFFE. This indicates a wrong byte order.".to_string()));
        }

        return String::from_utf16(sl).map_err(|_e| Error::new(ErrorKind::InvalidData, "invalid utf-16 data"));
    }

    #[cfg(target_endian = "big")]
    fn read_string_utf16_be(&mut self, size_in_characters: usize) -> io::Result<String> {
        if size_in_characters == 0 {
            return Ok("".to_string());
        }

        let mut data = vec![0u8; size_in_characters<<1];
        self.read_exact(data.as_mut_slice())?;

        let sl :&[u16] = unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr().cast(), size_in_characters) };

        if sl[0] == 0xFFFE {
            return Err(Error::new(ErrorKind::InvalidData, "Encountered byte order mark 0xFFFE. This indicates a wrong byte order.".to_string()));
        }

        return String::from_utf16(sl).map_err(|_e| Error::new(ErrorKind::InvalidData, "invalid utf-16 data"));
    }

    #[cfg(target_endian = "little")]
    fn read_string_utf16_le(&mut self, size_in_characters: usize) -> io::Result<String> {
        if size_in_characters == 0 {
            return Ok("".to_string());
        }

        let mut data = vec![0u8; size_in_characters<<1];
        self.read_exact(data.as_mut_slice())?;

        let sl :&[u16] = unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr().cast(), size_in_characters) };

        if sl[0] == 0xFFFE {
            return Err(Error::new(ErrorKind::InvalidData, "Encountered byte order mark 0xFFFE. This indicates a wrong byte order.".to_string()));
        }

        return String::from_utf16(sl).map_err(|_e| Error::new(ErrorKind::InvalidData, "invalid utf-16 data"));
    }

    #[cfg(target_endian = "big")]
    fn read_string_utf16_le(&mut self, size_in_characters: usize) -> io::Result<String> {
        if size_in_characters == 0 {
            return Ok("".to_string());
        }

        let mut data = vec![0u8; size_in_characters<<1];
        self.read_exact(data.as_mut_slice())?;

        let sl :&mut [u16] = unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr().cast(), size_in_characters) };
        for i in 0 .. sl.len() {
            sl[i] = sl[i].to_le();
        }

        if sl[0] == 0xFFFE {
            return Err(Error::new(ErrorKind::InvalidData, "Encountered byte order mark 0xFFFE. This indicates a wrong byte order.".to_string()));
        }

        return String::from_utf16(sl).map_err(|_e| Error::new(ErrorKind::InvalidData, "invalid utf-16 data"));
    }

    #[cfg(target_endian = "big")]
    fn read_string_utf32_be(&mut self, size_in_characters: usize) -> io::Result<String> {
        if size_in_characters == 0 {
            return Ok("".to_string());
        }
        let mut data = vec![0u32; size_in_characters];
        let sl : &mut [u8] = unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr().cast(), size_in_characters * size_of::<char>()) };
        self.read_exact(sl)?;

        if data[0] == 0xFFFE0000u32 {
            return Err(Error::new(ErrorKind::InvalidData, "Encountered byte order mark 0xFFFE. This indicates a wrong byte order.".to_string()));
        }

        for i in 0 .. data.len() {
            let cur = data[i];

            if char::from_u32(cur).is_none() {
                return Err(Error::new(ErrorKind::InvalidData, format!("{} is not a valid unicode codepoint.", cur)));
            }
        }

        let sl : &mut [char] = unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr().cast(), size_in_characters) };
        let str : String = sl.iter().collect();
        return Ok(str);
    }

    #[cfg(target_endian = "big")]
    fn read_string_utf32_le(&mut self, size_in_characters: usize) -> io::Result<String> {
        if size_in_characters == 0 {
            return Ok("".to_string());
        }
        let mut data = vec![0u32; size_in_characters];
        let sl : &mut [u8] = unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr().cast(), size_in_characters * 4) };
        self.read_exact(sl)?;

        if data[0].to_le() == 0xFFFE0000u32 {
            return Err(Error::new(ErrorKind::InvalidData, "Encountered byte order mark 0xFFFE. This indicates a wrong byte order.".to_string()));
        }

        for i in 0 .. data.len() {
            let cur = data[i].to_le();
            data[i] = cur;

            if char::from_u32(cur).is_none() {
                return Err(Error::new(ErrorKind::InvalidData, format!("{} is not a valid unicode codepoint.", cur)));
            }
        }

        let sl : &mut [char] = unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr().cast(), size_in_characters) };
        let str : String = sl.iter().collect();
        return Ok(str);
    }

    #[cfg(target_endian = "little")]
    fn read_string_utf32_be(&mut self, size_in_characters: usize) -> io::Result<String> {
        if size_in_characters == 0 {
            return Ok("".to_string());
        }
        let mut data = vec![0u32; size_in_characters];
        let sl : &mut [u8] = unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr().cast(), size_in_characters * 4) };
        self.read_exact(sl)?;

        if data[0].to_be() == 0xFFFE0000u32 {
            return Err(Error::new(ErrorKind::InvalidData, "Encountered byte order mark 0xFFFE. This indicates a wrong byte order.".to_string()));
        }

        for i in 0 .. data.len() {
            let cur = data[i].to_be();
            data[i] = cur;

            if char::from_u32(cur).is_none() {
                return Err(Error::new(ErrorKind::InvalidData, format!("{} is not a valid unicode codepoint.", cur)));
            }
        }

        let sl : &mut [char] = unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr().cast(), size_in_characters) };
        let str : String = sl.iter().collect();
        return Ok(str);
    }

    #[cfg(target_endian = "little")]
    fn read_string_utf32_le(&mut self, size_in_characters: usize) -> io::Result<String> {
        if size_in_characters == 0 {
            return Ok("".to_string());
        }
        let mut data = vec![0u32; size_in_characters];
        let sl : &mut [u8] = unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr().cast(), size_in_characters * size_of::<char>()) };
        self.read_exact(sl)?;

        if data[0] == 0xFFFE0000u32 {
            return Err(Error::new(ErrorKind::InvalidData, "Encountered byte order mark 0xFFFE. This indicates a wrong byte order.".to_string()));
        }

        for i in 0 .. data.len() {
            let cur = data[i];

            if char::from_u32(cur).is_none() {
                return Err(Error::new(ErrorKind::InvalidData, format!("{} is not a valid unicode codepoint.", cur)));
            }
        }

        let sl : &mut [char] = unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr().cast(), size_in_characters) };
        let str : String = sl.iter().collect();
        return Ok(str);
    }

    fn read_java_data_input_utf(&mut self) -> io::Result<String> {
        let mut buf = [0u8; 2];
        self.read_exact(buf.as_mut_slice())?;
        //this is always big endian in java
        let byte_count = (buf[0] as u16 >> 8 | buf[1] as u16) as usize;

        let mut buf = vec![0u8; byte_count];
        self.read_exact(buf.as_mut_slice())?;

        //This is optimistic alloc and works if we only send ascii;
        let mut characters: Vec<u16> = Vec::with_capacity(byte_count);

        let mut index = 0usize;
        while index < buf.len() {
            let c = buf[index] as u32;

            match c >> 4 {
                0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                    characters.push(c as u16);
                    index += 1;
                }
                12 | 13 => {
                    if index + 2 > buf.len() {
                        return Err(Error::new(ErrorKind::InvalidData, "Invalid input"));
                    }
                    let c2 = buf[index + 1] as u32;
                    index += 2;
                    if (c2 & 0xC0) != 0x80 {
                        return Err(Error::new(ErrorKind::InvalidData, "Invalid input"));
                    }

                    let v = ((c & 0x1F) << 6) | (c2 & 0x3F);
                    characters.push(v as u16)
                }
                14 => {
                    if index + 3 > buf.len() {
                        return Err(Error::new(ErrorKind::InvalidData, "Invalid input"));
                    }
                    let c2 = buf[index + 1] as u32;
                    let c3 = buf[index + 2] as u32;
                    index += 3;
                    if ((c2 & 0xC0) != 0x80) || ((c3 & 0xC0) != 0x80) {
                        return Err(Error::new(ErrorKind::InvalidData, "Invalid input"));
                    }
                    let v = ((c & 0x0F) << 12) | ((c2 & 0x3F) << 6) | ((c3 & 0x3F) << 0);
                    characters.push(v as u16)
                }
                _ => {
                    return Err(Error::new(ErrorKind::InvalidData, "Invalid input"));
                }
            }
        }

        let result = String::from_utf16(&characters).map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid input"))?;
        Ok(result)
    }
}

mod private {
    use std::io::Read;

    impl <T> Sealed for T where T: Read {}
    pub trait Sealed {

    }
}
