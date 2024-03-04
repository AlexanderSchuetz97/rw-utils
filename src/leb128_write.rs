use std::io;
use std::io::{Error, ErrorKind, Write};
use std::mem::size_of;

///
/// Trait that provides various methods to write leb128 (little endian base 128) encoded numbers.
/// Automatically implemented for all implementations of io::Write.
/// This trait is sealed and cannot be implemented manually.
///
pub trait Leb128Write : private::Sealed {

    ///
    /// Writes a u16 as a unsigned leb128.
    ///
    fn write_leb128_u16(&mut self, value: u16) -> io::Result<()>;

    ///
    /// Writes a i16 as a signed leb128.
    ///
    fn write_leb128_i16(&mut self, value: i16) -> io::Result<()>;

    ///
    /// Writes a u32 as a unsigned leb128.
    ///
    fn write_leb128_u32(&mut self, value: u32) -> io::Result<()>;

    ///
    /// Writes a i32 as a signed leb128.
    ///
    fn write_leb128_i32(&mut self, value: i32) -> io::Result<()>;

    ///
    /// Writes a u64 as a unsigned leb128.
    ///
    fn write_leb128_u64(&mut self, value: u64) -> io::Result<()>;

    ///
    /// Writes a i64 as a signed leb128.
    ///
    fn write_leb128_i64(&mut self, value: i64) -> io::Result<()>;

    ///
    /// Writes a u128 as a unsigned leb128.
    ///
    fn write_leb128_u128(&mut self, value: u128) -> io::Result<()>;

    ///
    /// Writes a i128 as a signed leb128.
    ///
    fn write_leb128_i128(&mut self, value: i128) -> io::Result<()>;

    ///
    /// Write a signed leb128 of arbitrary length.
    /// The input data is always treated as little endian.
    ///
    fn write_leb128_large_signed(&mut self, value: &Vec<u8>) -> io::Result<()>;

    ///
    /// Write an unsigned leb128 of arbitrary length.
    /// The input data is always treated as little endian.
    ///
    fn write_leb128_large_unsigned(&mut self, value: &Vec<u8>) -> io::Result<()>;
}

fn pump_leb128_data<T: Write>(s: &mut T, value: &Vec<u8>, last: usize) -> io::Result<(u64, usize)> {
    let mut acc: u64 = 0;
    let mut size: usize = 0;
    for x in value.as_slice()[0..last].iter() {
        if size >= 56 {
            while size > 0 {
                size -= 7;
                let next = ((acc & 0b0111_1111u64) as u8) | 0b1000_0000u8 ;
                acc >>= 7;
                s.write_all(&[next])?;
            }
            acc = 0;
        }

        acc |= (*x as u64) << size;
        size += 8;
    }

    return Ok((acc,size));
}

macro_rules! write_unsigned_leb {
    ($type:ty, $name:ident) => {
        fn $name(&mut self, value: $type) -> io::Result<()> {
            if value == 0 {
                return self.write_all(&[0]);
            }

            let mut acc : $type = value;

            let mut size = size_of::<$type>()*8;
            loop {
                size = size.saturating_sub(7);
                let mut next = (acc & 0b0111_1111) as u8;
                acc >>= 7;

                if acc == 0 || size == 0 {
                    return self.write_all(&[next]);
                }

                next |= 0b1000_0000;

                self.write_all(&[next])?;
            }
        }
    }
}

macro_rules! write_signed_leb {
    ($type:ty, $helper:ty, $name:ident) => {
        fn $name(&mut self, value: $type) -> io::Result<()> {
        if value == 0 {
            return self.write_all(&[0]);
        }

        let mut acc = value as $helper;
        let mut size = size_of::<$helper>()*8;
        let mut acc_eq : $helper = !0;

        loop {
            size = size.saturating_sub(7);
            let mut next = (acc & 0b0111_1111) as u8;

            if size == 0 {
                return self.write_all(&[next]);
            }

            acc >>= 7;
            acc_eq >>= 7;

            if next & 0b0100_0000 != 0 {
                if acc_eq == acc {
                    return self.write_all(&[next]);
                }
            } else {
                if acc == 0 {
                    return self.write_all(&[next]);
                }
            }

            next |= 0b1000_0000;

            self.write_all(&[next])?;
        }
        }
    }
}


impl <T> Leb128Write for T where T: Write {

    write_unsigned_leb!(u16, write_leb128_u16);
    write_unsigned_leb!(u32, write_leb128_u32);
    write_unsigned_leb!(u64, write_leb128_u64);
    write_unsigned_leb!(u128, write_leb128_u128);

    write_signed_leb!(i16, u16, write_leb128_i16);
    write_signed_leb!(i32, u32, write_leb128_i32);
    write_signed_leb!(i64, u64, write_leb128_i64);
    write_signed_leb!(i128, u128, write_leb128_i128);


    fn write_leb128_large_signed(&mut self, value: &Vec<u8>) -> io::Result<()> {
        if value.len() == 0 {
            return Err(Error::new(ErrorKind::Other, "value parameter has len of 0"));
        }

        let mut last = value.len();
        let is_negative = value[last-1] & 0b1000_0000u8 != 0;
        if is_negative {
            while last > 0 && value[last-1] == u8::MAX {
                last -= 1;
            }
        } else {
            while last > 0 && value[last-1] == 0{
                last -= 1;
            }
        }

        if last == 0 {
            if is_negative {
                return self.write_all(&[0b0111_1111u8]);
            }
            return self.write_all(&[0]);
        }

        let (mut acc, mut size) = pump_leb128_data(self, value, last)?;

        let mut acc_cmp : u64 = !0;

        if is_negative {
            acc |= !((1 << size)-1)
        }

        loop {
            size = size.saturating_sub(7);
            let mut next = (acc & 0b0111_1111u64) as u8;
            acc >>= 7;
            acc_cmp >>= 7;

            if size == 0 {
                return self.write_all(&[next]);
            }

            if is_negative {
                if acc == acc_cmp && next & 0b0100_0000 != 0 {
                    return self.write_all(&[next]);
                }
            } else {
                if acc == 0 && next & 0b0100_0000 == 0 {
                    return self.write_all(&[next]);
                }
            }

            next |= 0b1000_0000u8;

            self.write_all(&[next])?;
        }
    }

    fn write_leb128_large_unsigned(&mut self, value: &Vec<u8>) -> io::Result<()> {
        if value.len() == 0 {
            return Err(Error::new(ErrorKind::Other, "value parameter has len of 0"));
        }

        let mut last = value.len();
        while last > 0 && value[last-1] == 0 {
            last -= 1;
        }

        if last == 0 {
            return self.write_all(&[0]);
        }

        let (mut acc, mut size) = pump_leb128_data(self, value, last)?;

        loop {
            size = size.saturating_sub(7);
            let mut next = (acc & 0b0111_1111u64) as u8;
            acc >>= 7;

            if size == 0 || acc == 0 {
                return self.write_all(&[next]);
            }

            next |= 0b1000_0000u8;

            self.write_all(&[next])?;
        }
    }
}

mod private {
    use std::io::Write;

    impl <T> Sealed for T where T: Write {}
    pub trait Sealed {

    }
}
