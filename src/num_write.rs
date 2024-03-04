use std::io;
use std::io::Write;
use std::mem::size_of;

///
/// Trait that provides various methods to write numbers.
/// Automatically implemented for all implementations of io::Write.
///
pub trait NumWrite : private::Sealed {

    ///
    /// Writes one byte and treats 0 as false and any other value as true.
    ///
    fn write_bool(&mut self, value: bool) -> io::Result<()>;

    ///
    /// Writes 1 byte and treats it as an u8
    ///
    fn write_u8(&mut self, value: u8) -> io::Result<()>;

    ///
    /// Writes 2 bytes and treats them as an u16 in little endian byte order
    ///
    fn write_u16_le(&mut self, value: u16) -> io::Result<()>;

    ///
    /// Writes 2 bytes and treats them as an u16 in big endian byte order
    ///
    fn write_u16_be(&mut self, value: u16) -> io::Result<()>;

    ///
    /// Writes 2 bytes and treats them as an u16 in native byte order
    ///
    fn write_u16_ne(&mut self, value: u16) -> io::Result<()>;

    ///
    /// Writes 4 bytes and treats them as an u32 in little endian byte order
    ///
    fn write_u32_le(&mut self, value: u32) -> io::Result<()>;

    ///
    /// Writes 4 bytes and treats them as an u32 in big endian byte order
    ///
    fn write_u32_be(&mut self, value: u32) -> io::Result<()>;

    ///
    /// Writes 4 bytes and treats them as an u32 in native byte order
    ///
    fn write_u32_ne(&mut self, value: u32) -> io::Result<()>;

    ///
    /// Writes 8 bytes and treats them as an u64 in little endian byte order
    ///
    fn write_u64_le(&mut self, value: u64) -> io::Result<()>;

    ///
    /// Writes 8 bytes and treats them as an u64 in big endian byte order
    ///
    fn write_u64_be(&mut self, value: u64) -> io::Result<()>;

    ///
    /// Writes 8 bytes and treats them as an u64 in native byte order
    ///
    fn write_u64_ne(&mut self, value: u64) -> io::Result<()>;

    ///
    /// Writes 16 bytes and treats them as an u128 in little endian byte order
    ///
    fn write_u128_le(&mut self, value: u128) -> io::Result<()>;

    ///
    /// Writes 16 bytes and treats them as an u128 in big endian byte order
    ///
    fn write_u128_be(&mut self, value: u128) -> io::Result<()>;

    ///
    /// Writes 16 bytes and treats them as an u128 in native byte order
    ///
    fn write_u128_ne(&mut self, value: u128) -> io::Result<()>;

    ///
    /// Writes 1 byte and treats it as an i8
    ///
    fn write_i8(&mut self, value: i8) -> io::Result<()>;

    ///
    /// Writes 2 bytes and treats them as an i16 in little endian byte order
    ///
    fn write_i16_le(&mut self, value: i16) -> io::Result<()>;

    ///
    /// Writes 2 bytes and treats them as an i16 in big endian byte order
    ///
    fn write_i16_be(&mut self, value: i16) -> io::Result<()>;

    ///
    /// Writes 2 bytes and treats them as an u16 in native byte order
    ///
    fn write_i16_ne(&mut self, value: i16) -> io::Result<()>;

    ///
    /// Writes 4 bytes and treats them as an i32 in little endian byte order
    ///
    fn write_i32_le(&mut self, value: i32) -> io::Result<()>;

    ///
    /// Writes 4 bytes and treats them as an i32 in big endian byte order
    ///
    fn write_i32_be(&mut self, value: i32) -> io::Result<()>;

    ///
    /// Writes 4 bytes and treats them as an i32 in native byte order
    ///
    fn write_i32_ne(&mut self, value: i32) -> io::Result<()>;

    ///
    /// Writes 8 bytes and treats them as an i64 in little endian byte order
    ///
    fn write_i64_le(&mut self, value: i64) -> io::Result<()>;

    ///
    /// Writes 8 bytes and treats them as an i64 in big endian byte order
    ///
    fn write_i64_be(&mut self, value: i64) -> io::Result<()>;

    ///
    /// Writes 8 bytes and treats them as an u64 in native byte order
    ///
    fn write_i64_ne(&mut self, value: i64) -> io::Result<()>;

    ///
    /// Writes 16 bytes and treats them as an u128 in little endian byte order
    ///
    fn write_i128_le(&mut self, value: i128) -> io::Result<()>;

    ///
    /// Writes 16 bytes and treats them as an u128 in big endian byte order
    ///
    fn write_i128_be(&mut self, value: i128) -> io::Result<()>;

    ///
    /// Writes 16 bytes and treats them as an u128 in native byte order
    ///
    fn write_i128_ne(&mut self, value: i128) -> io::Result<()>;

    ///
    /// Writes 4 bytes and treats them as an f32 in little endian byte order
    ///
    fn write_f32_le(&mut self, value: f32) -> io::Result<()>;

    ///
    /// Writes 4 bytes and treats them as an f32 in big endian byte order
    ///
    fn write_f32_be(&mut self, value: f32) -> io::Result<()>;

    ///
    /// Writes 4 bytes and treats them as an f32 in native byte order
    ///
    fn write_f32_ne(&mut self, value: f32) -> io::Result<()>;

    ///
    /// Writes 8 bytes and treats them as an f64 in little endian byte order
    ///
    fn write_f64_le(&mut self, value: f64) -> io::Result<()>;

    ///
    /// Writes 8 bytes and treats them as an f64 in big endian byte order
    ///
    fn write_f64_be(&mut self, value: f64) -> io::Result<()>;

    ///
    /// Writes 8 bytes and treats them as an f64 in native byte order
    ///
    fn write_f64_ne(&mut self, value: f64) -> io::Result<()>;

    ///
    /// Writes the entire slice.
    ///
    fn write_u8_slice(&mut self, slice: &[u8]) -> io::Result<()>;

    ///
    /// Writes the entire slice.
    ///
    fn write_i8_slice(&mut self, slice: &[i8]) -> io::Result<()>;

    ///
    /// Writes the entire slice. Each element is encoded in little endian.
    ///
    fn write_u16_slice_le(&mut self, slice: &[u16]) -> io::Result<()>;

    ///
    /// Writes the entire slice. Each element is encoded in big endian.
    ///
    fn write_u16_slice_be(&mut self, slice: &[u16]) -> io::Result<()>;

    ///
    /// Writes the entire slice. Each element is encoded in native endian.
    ///
    fn write_u16_slice_ne(&mut self, slice: &[u16]) -> io::Result<()>;

    ///
    /// Writes the entire slice. Each element is encoded in little endian.
    ///
    fn write_u32_slice_le(&mut self, slice: &[u32]) -> io::Result<()>;

    ///
    /// Writes the entire slice. Each element is encoded in big endian.
    ///
    fn write_u32_slice_be(&mut self, slice: &[u32]) -> io::Result<()>;
    ///
    /// Writes the entire slice. Each element is encoded in native endian.
    ///
    fn write_u32_slice_ne(&mut self, slice: &[u32]) -> io::Result<()>;

    ///
    /// Writes the entire slice. Each element is encoded in little endian.
    ///
    fn write_u64_slice_le(&mut self, slice: &[u64]) -> io::Result<()>;
    ///
    /// Writes the entire slice. Each element is encoded in big endian.
    ///
    fn write_u64_slice_be(&mut self, slice: &[u64]) -> io::Result<()>;
    ///
    /// Writes the entire slice. Each element is encoded in native endian.
    ///
    fn write_u64_slice_ne(&mut self, slice: &[u64]) -> io::Result<()>;

    ///
    /// Writes the entire slice. Each element is encoded in little endian.
    ///
    fn write_u128_slice_le(&mut self, slice: &[u128]) -> io::Result<()>;
    ///
    /// Writes the entire slice. Each element is encoded in big endian.
    ///
    fn write_u128_slice_be(&mut self, slice: &[u128]) -> io::Result<()>;
    ///
    /// Writes the entire slice. Each element is encoded in native endian.
    ///
    fn write_u128_slice_ne(&mut self, slice: &[u128]) -> io::Result<()>;

    ///
    /// Writes the entire slice. Each element is encoded in little endian.
    ///
    fn write_i16_slice_le(&mut self, slice: &[i16]) -> io::Result<()>;
    ///
    /// Writes the entire slice. Each element is encoded in big endian.
    ///
    fn write_i16_slice_be(&mut self, slice: &[i16]) -> io::Result<()>;
    ///
    /// Writes the entire slice. Each element is encoded in native endian.
    ///
    fn write_i16_slice_ne(&mut self, slice: &[i16]) -> io::Result<()>;

    ///
    /// Writes the entire slice. Each element is encoded in little endian.
    ///
    fn write_i32_slice_le(&mut self, slice: &[i32]) -> io::Result<()>;
    ///
    /// Writes the entire slice. Each element is encoded in big endian.
    ///
    fn write_i32_slice_be(&mut self, slice: &[i32]) -> io::Result<()>;
    ///
    /// Writes the entire slice. Each element is encoded in native endian.
    ///
    fn write_i32_slice_ne(&mut self, slice: &[i32]) -> io::Result<()>;

    ///
    /// Writes the entire slice. Each element is encoded in little endian.
    ///
    fn write_i64_slice_le(&mut self, slice: &[i64]) -> io::Result<()>;
    ///
    /// Writes the entire slice. Each element is encoded in big endian.
    ///
    fn write_i64_slice_be(&mut self, slice: &[i64]) -> io::Result<()>;
    ///
    /// Writes the entire slice. Each element is encoded in native endian.
    ///
    fn write_i64_slice_ne(&mut self, slice: &[i64]) -> io::Result<()>;

    ///
    /// Writes the entire slice. Each element is encoded in little endian.
    ///
    fn write_i128_slice_le(&mut self, slice: &[i128]) -> io::Result<()>;
    ///
    /// Writes the entire slice. Each element is encoded in big endian.
    ///
    fn write_i128_slice_be(&mut self, slice: &[i128]) -> io::Result<()>;
    ///
    /// Writes the entire slice. Each element is encoded in native endian.
    ///
    fn write_i128_slice_ne(&mut self, slice: &[i128]) -> io::Result<()>;

    ///
    /// Writes the entire slice. Each element is encoded in little endian.
    ///
    fn write_f64_slice_le(&mut self, slice: &[f64]) -> io::Result<()>;
    ///
    /// Writes the entire slice. Each element is encoded in big endian.
    ///
    fn write_f64_slice_be(&mut self, slice: &[f64]) -> io::Result<()>;
    ///
    /// Writes the entire slice. Each element is encoded in native endian.
    ///
    fn write_f64_slice_ne(&mut self, slice: &[f64]) -> io::Result<()>;

    ///
    /// Writes the entire slice. Each element is encoded in little endian.
    ///
    fn write_f32_slice_le(&mut self, slice: &[f32]) -> io::Result<()>;
    ///
    /// Writes the entire slice. Each element is encoded in big endian.
    ///
    fn write_f32_slice_be(&mut self, slice: &[f32]) -> io::Result<()>;
    ///
    /// Writes the entire slice. Each element is encoded in native endian.
    ///
    fn write_f32_slice_ne(&mut self, slice: &[f32]) -> io::Result<()>;

    ///
    /// Writes the entire vec.
    ///
    fn write_u8_vec(&mut self, slice: &Vec<u8>) -> io::Result<()>;

    ///
    /// Writes the entire vec.
    ///
    fn write_i8_vec(&mut self, slice: &Vec<i8>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in little endian.
    ///
    fn write_u16_vec_le(&mut self, slice: &Vec<u16>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in big endian.
    ///
    fn write_u16_vec_be(&mut self, slice: &Vec<u16>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in native endian.
    ///
    fn write_u16_vec_ne(&mut self, slice: &Vec<u16>) -> io::Result<()>;
    ///
    /// Writes the entire vec. Each element is encoded in little endian.
    ///
    fn write_u32_vec_le(&mut self, slice: &Vec<u32>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in big endian.
    ///
    fn write_u32_vec_be(&mut self, slice: &Vec<u32>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in native endian.
    ///
    fn write_u32_vec_ne(&mut self, slice: &Vec<u32>) -> io::Result<()>;
    ///
    /// Writes the entire vec. Each element is encoded in little endian.
    ///
    fn write_u64_vec_le(&mut self, slice: &Vec<u64>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in big endian.
    ///
    fn write_u64_vec_be(&mut self, slice: &Vec<u64>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in native endian.
    ///
    fn write_u64_vec_ne(&mut self, slice: &Vec<u64>) -> io::Result<()>;
    ///
    /// Writes the entire vec. Each element is encoded in little endian.
    ///
    fn write_u128_vec_le(&mut self, slice: &Vec<u128>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in big endian.
    ///
    fn write_u128_vec_be(&mut self, slice: &Vec<u128>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in native endian.
    ///
    fn write_u128_vec_ne(&mut self, slice: &Vec<u128>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in little endian.
    ///
    fn write_i16_vec_le(&mut self, slice: &Vec<i16>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in big endian.
    ///
    fn write_i16_vec_be(&mut self, slice: &Vec<i16>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in native endian.
    ///
    fn write_i16_vec_ne(&mut self, slice: &Vec<i16>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in little endian.
    ///
    fn write_i32_vec_le(&mut self, slice: &Vec<i32>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in big endian.
    ///
    fn write_i32_vec_be(&mut self, slice: &Vec<i32>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in native endian.
    ///
    fn write_i32_vec_ne(&mut self, slice: &Vec<i32>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in little endian.
    ///
    fn write_i64_vec_le(&mut self, slice: &Vec<i64>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in big endian.
    ///
    fn write_i64_vec_be(&mut self, slice: &Vec<i64>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in native endian.
    ///
    fn write_i64_vec_ne(&mut self, slice: &Vec<i64>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in little endian.
    ///
    fn write_i128_vec_le(&mut self, slice: &Vec<i128>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in big endian.
    ///
    fn write_i128_vec_be(&mut self, slice: &Vec<i128>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in native endian.
    ///
    fn write_i128_vec_ne(&mut self, slice: &Vec<i128>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in little endian.
    ///
    fn write_f64_vec_le(&mut self, slice: &Vec<f64>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in big endian.
    ///
    fn write_f64_vec_be(&mut self, slice: &Vec<f64>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in native endian.
    ///
    fn write_f64_vec_ne(&mut self, slice: &Vec<f64>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in little endian.
    ///
    fn write_f32_vec_le(&mut self, slice: &Vec<f32>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in big endian.
    ///
    fn write_f32_vec_be(&mut self, slice: &Vec<f32>) -> io::Result<()>;

    ///
    /// Writes the entire vec. Each element is encoded in native endian.
    ///
    fn write_f32_vec_ne(&mut self, slice: &Vec<f32>) -> io::Result<()>;
}

macro_rules! define_endian_numeric_write_functions {
    ($type:ty, $le_name:ident, $be_name:ident, $ne_name:ident) => {
        #[cfg(target_endian = "big")]
        fn $le_name(&mut self, value: $type) -> io::Result<()> {
            self.write_all(value.to_le_bytes().as_slice())
        }

        #[cfg(target_endian = "little")]
        fn $le_name(&mut self, value: $type) -> io::Result<()> {
            return self.write_all(unsafe { std::slice::from_raw_parts::<u8>(((&value) as *const $type).cast(), size_of::<$type>()) });
        }

        #[cfg(target_endian = "big")]
        fn $be_name(&mut self, value: $type) -> io::Result<()> {
            return self.write_all(unsafe { std::slice::from_raw_parts::<u8>(((&value) as *const $type).cast(), size_of::<$type>()) });
        }

        #[cfg(target_endian = "little")]
        fn $be_name(&mut self, value: $type) -> io::Result<()> {
            self.write_all(value.to_be_bytes().as_slice())
        }

        fn $ne_name(&mut self, value: $type) -> io::Result<()> {
            return self.write_all(unsafe { std::slice::from_raw_parts::<u8>(((&value) as *const $type).cast(), size_of::<$type>()) });
        }
    };
}

macro_rules! define_endian_slice_write_functions {
    ($type:ty, $helper:ty, $le_name:ident, $be_name:ident, $ne_name:ident) => {
        #[cfg(target_endian = "little")]
        fn $le_name(&mut self, slice: &[$type]) -> io::Result<()> {
            let sl :&[u8] = unsafe { std::slice::from_raw_parts(slice.as_ptr().cast(), slice.len() * size_of::<$type>()) };
            self.write_all(sl)
        }

        #[cfg(target_endian = "big")]
        fn $le_name(&mut self, slice: &[$type]) -> io::Result<()> {
            let mut copy : Vec<$helper> = Vec::with_capacity(slice.len());
            let helper :&[$helper] = unsafe { std::slice::from_raw_parts(slice.as_ptr().cast(), slice.len()) };
            copy.copy_from_slice(helper);
            for i in 0 .. copy.len() {
                copy[i] = copy[i].to_le();
            }
            let sl :&[u8] = unsafe { std::slice::from_raw_parts(copy.as_ptr().cast(), copy.len() * size_of::<$type>()) };
            self.write_all(sl)
        }

        #[cfg(target_endian = "little")]
        fn $be_name(&mut self, slice: &[$type]) -> io::Result<()> {
            let mut copy : Vec<$helper> = Vec::with_capacity(slice.len());
            let helper :&[$helper] = unsafe { std::slice::from_raw_parts(slice.as_ptr().cast(), slice.len()) };
            copy.copy_from_slice(helper);
            for i in 0 .. copy.len() {
                copy[i] = copy[i].to_be();
            }
            let sl :&[u8] = unsafe { std::slice::from_raw_parts(copy.as_ptr().cast(), copy.len() * size_of::<$type>()) };
            self.write_all(sl)
        }

        #[cfg(target_endian = "big")]
        fn $be_name(&mut self, slice: &[$type]) -> io::Result<()> {
            let sl :&[u8] = unsafe { std::slice::from_raw_parts(slice.as_ptr().cast(), slice.len() * size_of::<$type>()) };
            self.write_all(sl)
        }

        fn $ne_name(&mut self, slice: &[$type]) -> io::Result<()> {
            let sl :&[u8] = unsafe { std::slice::from_raw_parts(slice.as_ptr().cast(), slice.len() * size_of::<$type>()) };
            self.write_all(sl)
        }
    };
}

macro_rules! define_endian_vec_write_functions {
    ($type:ty, $helper:ty, $le_name:ident, $be_name:ident, $ne_name:ident) => {
        #[cfg(target_endian = "little")]
        fn $le_name(&mut self, slice: &Vec<$type>) -> io::Result<()> {
            let sl :&[u8] = unsafe { std::slice::from_raw_parts(slice.as_ptr().cast(), slice.len() * size_of::<$type>()) };
            self.write_all(sl)
        }

        #[cfg(target_endian = "big")]
        fn $le_name(&mut self, slice: &Vec<$type>) -> io::Result<()> {
            let mut copy : Vec<$helper> = Vec::with_capacity(slice.len());
            let helper :&[$helper] = unsafe { std::slice::from_raw_parts(slice.as_ptr().cast(), slice.len()) };
            copy.copy_from_slice(helper);
            for i in 0 .. copy.len() {
                copy[i] = copy[i].to_le();
            }
            let sl :&[u8] = unsafe { std::slice::from_raw_parts(copy.as_ptr().cast(), copy.len() * size_of::<$type>()) };
            self.write_all(sl)
        }

        #[cfg(target_endian = "little")]
        fn $be_name(&mut self, slice: &Vec<$type>) -> io::Result<()> {
            let mut copy : Vec<$helper> = Vec::with_capacity(slice.len());
            let helper :&[$helper] = unsafe { std::slice::from_raw_parts(slice.as_ptr().cast(), slice.len()) };
            copy.copy_from_slice(helper);
            for i in 0 .. copy.len() {
                copy[i] = copy[i].to_be();
            }
            let sl :&[u8] = unsafe { std::slice::from_raw_parts(copy.as_ptr().cast(), copy.len() * size_of::<$type>()) };
            self.write_all(sl)
        }

        #[cfg(target_endian = "big")]
        fn $be_name(&mut self, slice: &Vec<$type>) -> io::Result<()> {
            let sl :&[u8] = unsafe { std::slice::from_raw_parts(slice.as_ptr().cast(), slice.len() * size_of::<$type>()) };
            self.write_all(sl)
        }

        fn $ne_name(&mut self, slice: &Vec<$type>) -> io::Result<()> {
            let sl :&[u8] = unsafe { std::slice::from_raw_parts(slice.as_ptr().cast(), slice.len() * size_of::<$type>()) };
            self.write_all(sl)
        }
    };
}

impl<T> NumWrite for T where T: Write {
    fn write_bool(&mut self, value: bool) -> io::Result<()> {
        if value {
            return self.write_all(&[1u8]);
        } else {
            return self.write_all(&[0u8]);
        }
    }

    fn write_u8(&mut self, value: u8) -> io::Result<()> {
        return self.write_all(&[value]);
    }

    fn write_i8(&mut self, value: i8) -> io::Result<()> {
        return self.write_all(&[value as u8]);
    }

    define_endian_numeric_write_functions!(u16, write_u16_le, write_u16_be, write_u16_ne);
    define_endian_numeric_write_functions!(u32, write_u32_le, write_u32_be, write_u32_ne);
    define_endian_numeric_write_functions!(u64, write_u64_le, write_u64_be, write_u64_ne);
    define_endian_numeric_write_functions!(u128, write_u128_le, write_u128_be, write_u128_ne);

    define_endian_numeric_write_functions!(i16, write_i16_le, write_i16_be, write_i16_ne);
    define_endian_numeric_write_functions!(i32, write_i32_le, write_i32_be, write_i32_ne);
    define_endian_numeric_write_functions!(i64, write_i64_le, write_i64_be, write_i64_ne);
    define_endian_numeric_write_functions!(i128, write_i128_le, write_i128_be, write_i128_ne);

    define_endian_numeric_write_functions!(f32, write_f32_le, write_f32_be, write_f32_ne);
    define_endian_numeric_write_functions!(f64, write_f64_le, write_f64_be, write_f64_ne);


    define_endian_slice_write_functions!(u16, u16, write_u16_slice_le, write_u16_slice_be, write_u16_slice_ne);
    define_endian_slice_write_functions!(u32, u32, write_u32_slice_le, write_u32_slice_be, write_u32_slice_ne);
    define_endian_slice_write_functions!(u64, u64, write_u64_slice_le, write_u64_slice_be, write_u64_slice_ne);
    define_endian_slice_write_functions!(u128, u128, write_u128_slice_le, write_u128_slice_be, write_u128_slice_ne);

    define_endian_slice_write_functions!(i16, i16, write_i16_slice_le, write_i16_slice_be, write_i16_slice_ne);
    define_endian_slice_write_functions!(i32, i32, write_i32_slice_le, write_i32_slice_be, write_i32_slice_ne);
    define_endian_slice_write_functions!(i64, i64, write_i64_slice_le, write_i64_slice_be, write_i64_slice_ne);
    define_endian_slice_write_functions!(i128, i128, write_i128_slice_le, write_i128_slice_be, write_i128_slice_ne);

    define_endian_slice_write_functions!(f32, u32, write_f32_slice_le, write_f32_slice_be, write_f32_slice_ne);
    define_endian_slice_write_functions!(f64, u64, write_f64_slice_le, write_f64_slice_be, write_f64_slice_ne);

    define_endian_vec_write_functions!(u16, u16, write_u16_vec_le, write_u16_vec_be, write_u16_vec_ne);
    define_endian_vec_write_functions!(u32, u32, write_u32_vec_le, write_u32_vec_be, write_u32_vec_ne);
    define_endian_vec_write_functions!(u64, u64, write_u64_vec_le, write_u64_vec_be, write_u64_vec_ne);
    define_endian_vec_write_functions!(u128, u128, write_u128_vec_le, write_u128_vec_be, write_u128_vec_ne);

    define_endian_vec_write_functions!(i16, i16, write_i16_vec_le, write_i16_vec_be, write_i16_vec_ne);
    define_endian_vec_write_functions!(i32, i32, write_i32_vec_le, write_i32_vec_be, write_i32_vec_ne);
    define_endian_vec_write_functions!(i64, i64, write_i64_vec_le, write_i64_vec_be, write_i64_vec_ne);
    define_endian_vec_write_functions!(i128, i128, write_i128_vec_le, write_i128_vec_be, write_i128_vec_ne);

    define_endian_vec_write_functions!(f32, u32, write_f32_vec_le, write_f32_vec_be, write_f32_vec_ne);
    define_endian_vec_write_functions!(f64, u64, write_f64_vec_le, write_f64_vec_be, write_f64_vec_ne);

    fn write_u8_slice(&mut self, slice: &[u8]) -> io::Result<()> {
        return self.write_all(slice);
    }

    fn write_i8_slice(&mut self, slice: &[i8]) -> io::Result<()> {
        let sl :&[u8] = unsafe { std::slice::from_raw_parts(slice.as_ptr().cast(), slice.len()) };
        return self.write_all(sl);
    }

    fn write_u8_vec(&mut self, value: &Vec<u8>) -> io::Result<()> {
        return self.write_all(value.as_slice());
    }

    fn write_i8_vec(&mut self, slice: &Vec<i8>) -> io::Result<()> {
        return self.write_i8_slice(slice.as_slice());
    }
}

mod private {
    use std::io::Write;

    impl <T> Sealed for T where T: Write {}
    pub trait Sealed {

    }
}
