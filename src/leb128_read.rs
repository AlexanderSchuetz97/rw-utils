use std::io;
use std::io::{ErrorKind, Read};
use std::mem::size_of;

///
/// Trait that provides various methods to read leb128 (little endian base 128) encoded numbers.
/// Automatically implemented for all implementations of io::Read.
/// This trait is sealed and cannot be implemented manually.
///
pub trait Leb128Read : private::Sealed {
    ///
    /// Reads a unsigned leb128 as u16.
    /// Fails if the leb128 doesn't fit.
    ///
    fn read_leb128_u16(&mut self) -> io::Result<u16>;

    ///
    /// Reads a signed leb128 as i16.
    /// Fails if the leb128 doesn't fit.
    ///
    fn read_leb128_i16(&mut self) -> io::Result<i16>;

    ///
    /// Reads a unsigned leb128 as u32.
    /// Fails if the leb128 doesn't fit.
    ///
    fn read_leb128_u32(&mut self) -> io::Result<u32>;

    ///
    /// Reads a signed leb128 as i32.
    /// Fails if the leb128 doesn't fit.
    ///
    fn read_leb128_i32(&mut self) -> io::Result<i32>;

    ///
    /// Reads a unsigned leb128 as u64.
    /// Fails if the leb128 doesn't fit.
    ///
    fn read_leb128_u64(&mut self) -> io::Result<u64>;

    ///
    /// Reads a signed leb128 as i64.
    /// Fails if the leb128 doesn't fit.
    ///
    fn read_leb128_i64(&mut self) -> io::Result<i64>;

    ///
    /// Reads a unsigned leb128 as u128.
    /// Fails if the leb128 doesn't fit.
    ///
    fn read_leb128_u128(&mut self) -> io::Result<u128>;

    ///
    /// Reads a signed leb128 as i128.
    /// Fails if the leb128 doesn't fit.
    ///
    fn read_leb128_i128(&mut self) -> io::Result<i128>;

    ///
    /// Reads a signed leb128 of arbitrary length.
    /// The returned value is always in little endian.
    /// The max_size parameter controls the maximum size of the returned Vec, not the amount of data to be read.
    ///
    fn read_leb128_large_signed(&mut self, max_size: usize) -> io::Result<Vec<u8>>;

    ///
    /// Reads an unsigned leb128 of arbitrary length.
    /// The returned value is always in little endian.
    /// The max_size parameter controls the maximum size of the returned Vec, not the amount of data to be read.
    ///
    fn read_leb128_large_unsigned(&mut self, max_size: usize) -> io::Result<Vec<u8>>;
}

fn next<T: Read>(s: &mut T) -> io::Result<u8> {
    let mut buf = [0u8];
    s.read_exact(buf.as_mut_slice())?;
    return Ok(buf[0]);
}

fn finish_large_leb_read(mut shift: u32, mut acc: u64, max_size: usize, mut result: Vec<u8>, is_negative: bool) -> io::Result<Vec<u8>> {
    while shift > 0 {
        shift = shift.saturating_sub(8);

        let to_push = (acc & 0xFFu64) as u8;

        if result.len()+1 > max_size {
            if shift != 0 {
                return Err(io::Error::new(ErrorKind::InvalidData, "leb128 larger than desired maximum size"));
            }

            if is_negative {
                if to_push == u8::MAX {
                    //0xFF leading in twos complement can be discarded.
                    return Ok(result);
                }
            } else {
                if to_push == u8::MIN {
                    //Leading 0 bytes are not needed and don't count.
                    return Ok(result);
                }
            }

            return Err(io::Error::new(ErrorKind::InvalidData, "leb128 larger than desired maximum size"));
        }
        result.push(to_push);
        acc >>= 8;
    }

    return Ok(result);
}

fn push_next_large_leb_block(result : &mut Vec<u8>, max_size: usize, acc: u64) -> io::Result<()>{
    if result.len()+7 > max_size {
        return Err(io::Error::new(ErrorKind::InvalidData, "leb128 larger than desired maximum size"));
    }
    result.push(((acc >> 0) & 0xFFu64) as u8);
    result.push(((acc >> 8) & 0xFFu64) as u8);
    result.push(((acc >> 16) & 0xFFu64) as u8);
    result.push(((acc >> 24) & 0xFFu64) as u8);
    result.push(((acc >> 32) & 0xFFu64) as u8);
    result.push(((acc >> 40) & 0xFFu64) as u8);
    result.push(((acc >> 48) & 0xFFu64) as u8);
    return Ok(());
}


macro_rules! read_unsigned_leb {
    ($type:ty, $name:ident, $err:expr) => {
        fn $name(&mut self) -> io::Result<$type> {
            let mut acc : $type = 0;
            let mut shift = 0u32;
            let size = (size_of::<$type>() * 8) as u32;
            loop {
                if shift >= size {
                    return Err(io::Error::new(ErrorKind::InvalidData, $err));
                }
                let n : u8 = next(self)?;
                acc |= ((n & 0b0111_1111u8) as $type) << shift;
                shift+=7;
                if n & 0b1000_0000u8 == 0 {
                    return Ok(acc);
                }
            }
        }
    }
}

macro_rules! read_signed_leb {
    ($type:ty, $helper:ty, $name:ident, $err:expr) => {
        fn $name(&mut self) -> io::Result<$type> {
            let mut acc : $helper = 0;
            let mut shift = 0u32;
            let size = (size_of::<$helper>() * 8) as u32;
            loop {
                if shift >= size {
                    return Err(io::Error::new(ErrorKind::InvalidData, $err));
                }
                let n : u8 = next(self)?;
                acc |= ((n & 0b0111_1111u8) as $helper) << shift;

                shift+=7;
                if n & 0b1000_0000u8 == 0 {
                    if n & 0b0100_0000u8 != 0 && shift < size{
                        let ext : $helper = 1;
                        acc |= !((ext << shift) -1);
                    }
                    return Ok(acc as $type);
                }
            }
        }
    }
}

impl <T> Leb128Read for T where T: Read {

    read_signed_leb!(i16, u16, read_leb128_i16, "leb128 larger than i16");
    read_signed_leb!(i32, u32, read_leb128_i32, "leb128 larger than i32");
    read_signed_leb!(i64, u64, read_leb128_i64, "leb128 larger than i64");
    read_signed_leb!(i128, u128, read_leb128_i128, "leb128 larger than i128");

    read_unsigned_leb!(u16, read_leb128_u16, "leb128 larger than u16");
    read_unsigned_leb!(u32, read_leb128_u32, "leb128 larger than u32");
    read_unsigned_leb!(u64, read_leb128_u64, "leb128 larger than u64");
    read_unsigned_leb!(u128, read_leb128_u128, "leb128 larger than u128");
    fn read_leb128_large_signed(&mut self, max_size: usize) -> io::Result<Vec<u8>> {
        let mut acc : u64 = 0;
        let mut shift = 0u32;
        let mut result : Vec<u8> = Vec::with_capacity(max_size);
        loop {
            if shift >= 54 {
                push_next_large_leb_block(&mut result, max_size, acc)?;
                shift = 0;
                acc = 0;
            }
            let n : u8 = next(self)?;
            acc |= ((n & 0b0111_1111u8) as u64) << shift;
            shift+=7;
            if n & 0b1000_0000u8 == 0 {
                if n & 0b0100_0000u8 != 0 {
                    acc |= !((1u64 << shift) -1);
                    return finish_large_leb_read(shift, acc, max_size, result, true);
                }

                return finish_large_leb_read(shift, acc, max_size, result, false);
            }
        }
    }

    fn read_leb128_large_unsigned(&mut self, max_size: usize) -> io::Result<Vec<u8>> {
        let mut acc : u64 = 0;
        let mut shift = 0u32;
        let mut result : Vec<u8> = Vec::with_capacity(max_size);
        loop {
            if shift >= 54 {
                push_next_large_leb_block(&mut result, max_size, acc)?;
                shift = 0;
                acc = 0;
            }
            let n : u8 = next(self)?;
            acc |= ((n & 0b0111_1111u8) as u64) << shift;
            shift+=7;
            if n & 0b1000_0000u8 == 0 {
                return finish_large_leb_read(shift, acc, max_size, result, false);
            }
        }
    }
}

mod private {
    use std::io::Read;

    impl <T> Sealed for T where T: Read {}
    pub trait Sealed {

    }
}
